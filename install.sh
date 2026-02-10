#!/usr/bin/env bash
set -euo pipefail

REPO_OWNER="lemuray"
REPO_NAME="rustfetch"
BINARY_NAME="rustfetch"

# this lists the most famous fetching tools to detect them in the shell config file
FETCH_TOOLS=(fastfetch screenfetch neofetch pfetch sysfetch hyfetch macchina ufetch nitch)

OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH="$(uname -m)"

# standardize architecture and OS names to match release naming
case "${ARCH}" in
  x86_64|amd64) ARCH="x86_64" ;;
  aarch64|arm64) ARCH="aarch64" ;;
  *) echo "Unsupported architecture: ${ARCH}" >&2; exit 1 ;;
esac

case "${OS}" in
  linux) OS="unknown-linux-gnu" ;;
  darwin) OS="apple-darwin" ;;
  *) echo "Unsupported OS: ${OS}" >&2; exit 1 ;;
esac

detect_shell_config() {
  # detect shell and get its config file
  local shell_name
  shell_name="$(basename "${SHELL:-}")"

  case "${shell_name}" in
    bash) echo "${HOME}/.bashrc" ;;
    zsh) echo "${HOME}/.zshrc" ;;
    fish) echo "${HOME}/.config/fish/config.fish" ;;
    *) return 1 ;;
  esac
}

prompt_yes_no() {
  local prompt="$1"
  local reply

  # read from /dev/tty so prompts work even when the script is piped.
  # in case it is not available default to no not to silently modify
  # shell config files
  if [[ ! -t 0 ]] && [[ ! -e /dev/tty ]]; then
    echo "${prompt} [Y/n] (non-interactive, defaulting to no)"
    return 1
  fi

  while true; do
    read -r -p "${prompt} [Y/n] " reply < /dev/tty
    reply="$(printf '%s' "${reply}" | tr '[:upper:]' '[:lower:]')"
    case "${reply}" in
      ""|y|yes) return 0 ;;
      n|no) return 1 ;;
      *) echo "Invalid input" ;;
    esac
  done
}

is_standalone_fetch_line() {
  # regex vodoo magic to make sure the fetching tool we are detecting is a
  # standalone line and is not in some complicated logic, if it is, we skip
  # the automated shell integration

  local line="$1"
  local cmd="$2"
  local trimmed

  trimmed="${line#"${line%%[!$' \t']*}"}"
  trimmed="${trimmed%"${trimmed##*[!$' \t']}"}"

  # ignore empty lines and comments
  [[ -z "${trimmed}" ]] && return 1
  [[ "${trimmed}" == \#* ]] && return 1
  # if there's an inline comment, we shouldn't be touching it
  [[ "${trimmed}" == *"#"* ]] && return 1

  # check for any shell operator, in case there is one the user must have had a reason for it,
  # skip this one
  if [[ "${trimmed}" =~ [\;\|\&\<\>\`\$\(\)] ]]; then
    return 1
  fi

  # match the standalone command with flags if present
  if [[ "${trimmed}" =~ ^${cmd}([[:space:]]+--?[[:alnum:]_-]+(=[^[:space:]]+)*)*$ ]]; then
    return 0
  fi

  return 1
}

update_shell_config() {
  local config_file="$1"
  local install_dir="$2"

  if [[ -z "${config_file}" || ! -f "${config_file}" ]]; then
    echo "Skipping shell integration (no config file found)."
    return
  fi

  if [[ ! -w "${config_file}" ]]; then
    echo "Skipping shell integration (config file not writable): ${config_file}"
    return
  fi

  local -a lines=()
  local line
  local line_no=0

  # read file into an array so we can do inner edits and write back once
  while IFS= read -r line || [[ -n "${line}" ]]; do
    line_no=$((line_no + 1))
    lines+=("${line}")
  done < "${config_file}"

  local found=0
  local changed=0
  local needs_path=0
  local idx cmd

  # check if the install directory is already on PATH in the config file
  if [[ "${install_dir}" == "${HOME}/.local/bin" ]]; then
    local path_already_set=0
    for idx in "${!lines[@]}"; do
      if [[ "${lines[$idx]}" == *".local/bin"* ]] && [[ "${lines[$idx]}" == *"PATH"* ]]; then
        path_already_set=1
        break
      fi
    done
    if [[ "${path_already_set}" -eq 0 ]]; then
      needs_path=1
    fi
  fi

  # scan each line for standalone fetch commands, prompt at each match
  for idx in "${!lines[@]}"; do
    line="${lines[$idx]}"
    for cmd in "${FETCH_TOOLS[@]}"; do
      if is_standalone_fetch_line "${line}" "${cmd}"; then
        found=1
        if prompt_yes_no "${cmd} found in ${config_file} at line $((idx + 1)), would you like to replace it with rustfetch?"; then
          local indent
          # preserve indentation
          indent="${line%%[!$' \t']*}"
          lines[$idx]="${indent}rustfetch"
          changed=1
          echo "Replaced ${cmd} with rustfetch at line $((idx + 1)) in ${config_file}."
        else
          echo "Skipping shell integration in ${config_file}."
        fi
        break
      fi
    done
  done

  # if no fetching CLI tool is present, ask to append rustfetch
  if [[ "${found}" -eq 0 ]]; then
    if prompt_yes_no "No other fetching tool was found in ${config_file}, would you like rustfetch to run automatically when opening a terminal?"; then
      lines+=("rustfetch")
      changed=1
      echo "Added rustfetch to ${config_file}."
    else
      echo "Skipping shell integration in ${config_file}."
    fi
  fi

  # if we're going to write rustfetch into the config, ensure PATH includes
  # the install directory so the shell can actually find the binary
  if [[ "${changed}" -eq 1 ]] && [[ "${needs_path}" -eq 1 ]]; then
    # prepend the PATH export before everything else so rustfetch is found
    lines=("export PATH=\"\$HOME/.local/bin:\$PATH\"" "${lines[@]}")
    echo "Added ~/.local/bin to PATH in ${config_file}."
  fi

  # only rewrite the file if we changed anything
  if [[ "${changed}" -eq 1 ]]; then
    local tmp_file
    tmp_file="$(mktemp)"
    printf '%s\n' "${lines[@]}" > "${tmp_file}"
    mv "${tmp_file}" "${config_file}"
  fi
}

ASSET="${BINARY_NAME}-${ARCH}-${OS}.tar.gz"

# fetch latest release
LATEST_TAG="$(curl -fsSL "https://api.github.com/repos/${REPO_OWNER}/${REPO_NAME}/releases/latest" | \
  grep -E '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')"

if [[ -z "${LATEST_TAG}" ]]; then
  echo "Could not determine latest release tag." >&2
  exit 1
fi

TMP_DIR="$(mktemp -d)"
curl -fsSL "https://github.com/${REPO_OWNER}/${REPO_NAME}/releases/download/${LATEST_TAG}/${ASSET}" \
  -o "${TMP_DIR}/${ASSET}"

tar -xzf "${TMP_DIR}/${ASSET}" -C "${TMP_DIR}"

# fnstall to /usr/local/bin (fallback: ~/.local/bin)
INSTALL_DIR="/usr/local/bin"
if [[ ! -w "${INSTALL_DIR}" ]]; then
  INSTALL_DIR="${HOME}/.local/bin"
  mkdir -p "${INSTALL_DIR}"
fi

install -m 0755 "${TMP_DIR}/${BINARY_NAME}" "${INSTALL_DIR}/${BINARY_NAME}"

echo "Installed ${BINARY_NAME} to ${INSTALL_DIR}"
echo "Run: ${BINARY_NAME} --help"

# optional shell integration to replace or add rustfetch on terminal start
if CONFIG_FILE="$(detect_shell_config)"; then
  update_shell_config "${CONFIG_FILE}" "${INSTALL_DIR}"
else
  echo "Skipping shell integration (unsupported shell)."
fi
