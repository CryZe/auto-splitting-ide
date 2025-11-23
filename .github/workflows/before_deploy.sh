#!/bin/bash

set -ex

main() {
    local tag=$(git tag --points-at HEAD)
    local src=$(pwd) \
          stage=

    if [[ "$OS_NAME" =~ ^macos\-.*$ ]]; then
        stage=$(mktemp -d -t tmp)
    else
        stage=$(mktemp -d)
    fi

    if [ "$OS_NAME" = "ubuntu-latest" ]; then
        cp target/$TARGET/max-opt/auto-splitting-ide $stage/auto-splitting-ide
    elif [[ "$OS_NAME" =~ ^macos\-.*$ ]]; then
        cp target/$TARGET/max-opt/auto-splitting-ide $stage/auto-splitting-ide
    elif [ "$OS_NAME" = "windows-latest" ]; then
        cp target/$TARGET/max-opt/auto-splitting-ide.exe $stage/auto-splitting-ide.exe
    fi

    cd $stage
    if [ "$OS_NAME" = "windows-latest" ]; then
        7z a $src/auto-splitting-ide-$tag-$RELEASE_TARGET.zip *
    else
        tar czf $src/auto-splitting-ide-$tag-$RELEASE_TARGET.tar.gz *
    fi
    cd $src

    rm -rf $stage
}

main
