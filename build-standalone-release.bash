#!/bin/bash

VERSION=$(grep ^version Cargo.toml | awk '{ gsub(/"/, "", $3); print $3; }')
docker run -it --rm -v $(pwd):/work -w /work -u 1000 rust cargo build --release --features=onedir
PKG=chamkho-$VERSION-linux-amd64
rm -rf $PKG && mkdir $PKG
cp target/release/wordcut $PKG
cp data/* $PKG
cp LICE* $PKG
tar -cvvzf $PKG.tar.gz $PKG
