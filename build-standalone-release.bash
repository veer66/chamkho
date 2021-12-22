#!/bin/bash

VERSION=$(grep ^version Cargo.toml | awk '{ gsub(/"/, "", $3); print $3; }')
docker run -it --rm -v $(pwd):/work -w /work -u 1000 rust cargo build --release --features=onedir
PKG=chamkho-$VERSION-linux-amd64
rm -rf $PKG && mkdir $PKG
cp target/release/wordcut $PKG
cp data/* $PKG
cp LICE* $PKG
tar -cvvzf $PKG.tar.gz $PKG

PKGWIN=chamkho-$VERSION-windows-amd64
docker build -t rustwin -f Dockerfile.windows .
docker run -it --rm -v $(pwd):/work -w /work -u 1000 rustwin cargo build --release --features=onedir --target=x86_64-pc-windows-gnu
rm -rf $PKGWIN && mkdir $PKGWIN
cp target/x86_64-pc-windows-gnu/release/wordcut.exe $PKGWIN
cp data/* $PKGWIN
cp LICE* $PKGWIN
docker build -t zip -f Dockerfile.zip .
docker run -it --rm -v $(pwd):/work -w /work -u 1000 zip zip -r $PKGWIN.zip $PKGWIN
