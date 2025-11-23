#!/bin/bash

set -ex

main() {
    local cargo=cross
    if [ "$SKIP_CROSS" = "skip" ]; then
        cargo=cargo
    fi
    local release_flag=""
    local target_folder="debug"
    if [ "$IS_DEPLOY" = "true" ]; then
        release_flag="--profile max-opt"
        target_folder="max-opt"
    fi

    if [ -z "$FEATURES" ]; then
        FEATURE_FLAGS="--no-default-features --features desktop"
    else
        FEATURE_FLAGS="--no-default-features --features desktop,$FEATURES"
    fi

    $cargo build --target $TARGET $release_flag $FEATURE_FLAGS
}

main
