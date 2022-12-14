#!/bin/bash

VERSION=$(grep ^version Cargo.toml | awk '{ gsub(/"/, "", $3); print $3; }')

function build_pkg () {
    target=$1
    name=$2
    podman run -it --rm -v $(pwd):/work -w /work rust /bin/bash -c "rustup target add $target; cargo build --release --features=onedir --target=$target"
    PKG=chamkho-$VERSION-$name
    rm -rf $PKG && mkdir $PKG
    cp target/$target/release/wordcut $PKG
    cp data/* $PKG
    cp LICE* $PKG
    tar -cvvzf $PKG.tar.gz $PKG
}

#build_pkg x86_64-unknown-linux-musl linux-x64_64-musl
build_pkg x86_64-pc-windows-gnu windows-x86_64
build_pkg wasm32-wasi wasm32-wasi
# PKGWIN=chamkho-$VERSION-windows-x86_64
# docker build -t rustwin -f Dockerfile.windows .
# docker run -it --rm -v $(pwd):/work -w /work -u 1000 rustwin cargo build --release --features=onedir --target=x86_64-pc-windows-gnu
# rm -rf $PKGWIN && mkdir $PKGWIN
# cp target/x86_64-pc-windows-gnu/release/wordcut.exe $PKGWIN
# cp data/* $PKGWIN
# cp LICE* $PKGWIN
# docker build -t zip -f Dockerfile.zip .
# docker run -it --rm -v $(pwd):/work -w /work -u 1000 zip zip -r $PKGWIN.zip $PKGWIN

# wasm32-wasi
