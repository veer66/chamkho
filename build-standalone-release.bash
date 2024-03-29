#!/bin/bash

VERSION=$(grep ^version Cargo.toml | awk '{ gsub(/"/, "", $3); print $3; }')

function build_pkg () {
    target=$1
    name=$2
    podman run -it --rm -v $(pwd):/work -w /work rust /bin/bash -c "apt-get update -y && apt-get install -y gcc-mingw-w64-x86-64-win32 && rustup target add $target; cargo build --release --features=onedir --target=$target"
    PKG=chamkho-$VERSION-$name
    rm -rf $PKG && mkdir $PKG
    cp target/$target/release/wordcut* $PKG
    cp data/* $PKG
    cp LICE* $PKG
    tar -cvvzf $PKG.tar.gz $PKG
}

build_pkg x86_64-pc-windows-gnu windows-x86_64
build_pkg wasm32-wasi wasm32-wasi
build_pkg x86_64-unknown-linux-musl linux-x64_64-musl
