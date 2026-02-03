#!/usr/bin/env bash
set -euo pipefail

REPO_OWNER="lemuray"
REPO_NAME="rustfetch"
BINARY_NAME="rustfetch"

# Detect OS and architecture
OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH="$(uname -m)"

# Normalize architecture names to match release asset naming
case "${ARCH}" in
  x86_64|amd64) ARCH="x86_64" ;;
  aarch64|arm64) ARCH="aarch64" ;;
  *) echo "Unsupported architecture: ${ARCH}" >&2; exit 1 ;;
esac

# Normalize OS names to match release asset naming
case "${OS}" in
  linux) OS="unknown-linux-gnu" ;;
  darwin) OS="apple-darwin" ;;
  *) echo "Unsupported OS: ${OS}" >&2; exit 1 ;;
esac

ASSET="${BINARY_NAME}-${ARCH}-${OS}.tar.gz"

# Fetch latest release
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

# Install to /usr/local/bin (or fallback to ~/.local/bin)
INSTALL_DIR="/usr/local/bin"
if [[ ! -w "${INSTALL_DIR}" ]]; then
  INSTALL_DIR="${HOME}/.local/bin"
  mkdir -p "${INSTALL_DIR}"
fi

install -m 0755 "${TMP_DIR}/${BINARY_NAME}" "${INSTALL_DIR}/${BINARY_NAME}"

echo "Installed ${BINARY_NAME} to ${INSTALL_DIR}"
echo "Run: ${BINARY_NAME} --help"