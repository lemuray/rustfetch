#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'EOF'
Simple wall-time benchmark for a command.

Usage:
  scripts/runtime.sh [--runs N] [--warmup N] -- <command> [args...]

Examples:
  first run "cargo build --release", then:
  scripts/runtime.sh --runs 30 --warmup 5 -- ./target/release/rustfetch

Environment overrides:
  RUNS=<N>      (default: 20)
  WARMUP=<N>    (default: 3)
EOF
}

runs="${RUNS:-20}"
warmup="${WARMUP:-3}"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --runs)
      runs="$2"
      shift 2
      ;;
    --warmup)
      warmup="$2"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    --)
      shift
      break
      ;;
    *)
      break
      ;;
  esac
done

if [[ $# -eq 0 ]]; then
  usage
  exit 1
fi

if ! [[ "$runs" =~ ^[0-9]+$ ]] || ! [[ "$warmup" =~ ^[0-9]+$ ]] || [[ "$runs" -eq 0 ]]; then
  echo "Error: --runs must be > 0 and --warmup must be >= 0" >&2
  exit 1
fi

cmd=("$@")

to_ms() {
  awk -v ns="$1" 'BEGIN { printf "%.3f", ns / 1000000 }'
}

measure_once_ns() {
  local start_ns end_ns
  start_ns=$(date +%s%N)
  "${cmd[@]}" >/dev/null
  end_ns=$(date +%s%N)
  echo $((end_ns - start_ns))
}

echo "Command: ${cmd[*]}"
echo "Warmup runs: $warmup"
echo "Measured runs: $runs"

for ((i = 1; i <= warmup; i++)); do
  measure_once_ns >/dev/null
done

sum_ns=0
min_ns=0
max_ns=0

for ((i = 1; i <= runs; i++)); do
  run_ns=$(measure_once_ns)

  if [[ "$i" -eq 1 ]]; then
    min_ns="$run_ns"
    max_ns="$run_ns"
  else
    (( run_ns < min_ns )) && min_ns="$run_ns"
    (( run_ns > max_ns )) && max_ns="$run_ns"
  fi

  sum_ns=$((sum_ns + run_ns))
  echo "Run $i: $(to_ms "$run_ns") ms"
done

avg_ns=$((sum_ns / runs))

echo
echo "Summary"
echo "-------"
echo "Average: $(to_ms "$avg_ns") ms"
echo "Min:     $(to_ms "$min_ns") ms"
echo "Max:     $(to_ms "$max_ns") ms"
