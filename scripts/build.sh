#!/bin/bash

set -e

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BUILDROOT="$ROOT/buildroot"

make -C "$BUILDROOT" O="$BUILDROOT/output" \
    BR2_DEFCONFIG="$ROOT/configs/aurelianos_defconfig" \
    "$ROOT/configs/aurelianos_defconfig"

make -C "$BUILDROOT" O="$BUILDROOT/output" -j"$(nproc)"

