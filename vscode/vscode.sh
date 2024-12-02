#!/bin/bash
set -euxo pipefail

pushd ..

trap popd EXIT

if ! sudo docker inspect --type container vscode.rust &>/dev/null; then

    if ! sudo docker inspect --type image vscode.rust &>/dev/null; then
    
        sudo docker buildx build \
            --no-cache \
            -t vscode.rust \
            -f vscode/Dockerfile .
    fi
    
    sudo docker create \
        --name vscode.rust \
        -v .:/workspace \
        -e DISPLAY \
        -v /tmp/.X11-unix:/tmp/.X11-unix \
        -u $(id -u) \
        --ipc host \
        vscode.rust
fi

xhost local:root

sudo docker start vscode.rust
