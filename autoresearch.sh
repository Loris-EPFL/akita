#!/usr/bin/env bash
set -euo pipefail

# Fixed-seed retained Dense controls and genuine RV64 Jolt legal-cut workloads.
# Both matrices require independent final verification and mutation rejection.
# The primary metric remains the Dense engine time, not recursive/VM parity.
root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
export RAYON_NUM_THREADS=1
export AKITA_PARALLEL=0
primary_seen=0

for segments in 1 2 3 256; do
    output="$(cargo run --manifest-path "$root/akita/Cargo.toml" \
        --release --offline --locked -p akita-pcs --no-default-features \
        --features transcript-blake2b --example accumulation_relation_benchmark \
        -- --segments "$segments")"
    metric_seen=0
    while IFS= read -r line; do
        case "$line" in
            'METRIC benchmark_prover_ms='*)
                value="${line#*=}"
                metric_seen=1
                if [[ "$segments" == 256 ]]; then
                    primary_value="$value"
                    primary_seen=1
                else
                    printf 'METRIC dense%s_prover_ms=%s\n' "$segments" "$value"
                fi
                ;;
            'METRIC benchmark_'*)
                metric="${line#METRIC benchmark_}"
                printf 'METRIC dense%s_%s\n' "$segments" "$metric"
                ;;
            *) printf 'DENSE%s %s\n' "$segments" "$line" ;;
        esac
    done <<< "$output"
    [[ "$metric_seen" == 1 ]] || { printf 'Missing executed prover metric for %s\n' "$segments" >&2; exit 1; }
done

# Required delivery gate: actual frontend, original RAM range, query relation
# and one complete final native proof, with no per-segment full PCS proofs.
for segments in 1 2 3 256; do
    output="$(cargo run --manifest-path "$root/jolt/Cargo.toml" \
        --release --offline --locked -p jolt-prover \
        --features indexed-accumulation --bin akita-indexed-accumulation \
        -- --segments "$segments")"
    metric_seen=0
    while IFS= read -r line; do
        case "$line" in
            'METRIC accumulation_prover_ms='*)
                metric_seen=1
                printf 'METRIC jolt%s_prover_ms=%s\n' "$segments" "${line#*=}"
                ;;
            'METRIC accumulation_'*)
                printf 'METRIC jolt%s_%s\n' "$segments" "${line#METRIC accumulation_}"
                ;;
            'METRIC per_segment_proofs='*|'METRIC final_proofs='*)
                printf 'METRIC jolt%s_%s\n' "$segments" "${line#METRIC }"
                ;;
            *) printf 'JOLT%s %s\n' "$segments" "$line" ;;
        esac
    done <<< "$output"
    [[ "$metric_seen" == 1 ]] || { printf 'Missing executed Jolt prover metric for %s\n' "$segments" >&2; exit 1; }
done

[[ "$primary_seen" == 1 ]] || { printf 'Missing 256-source benchmark metric\n' >&2; exit 1; }
printf 'METRIC accumulation_dense256_prover_ms=%s\n' "$primary_value"
