#!/bin/bash

VERSION=1.0.3
docker run -it --rm -v $(pwd):/work -w /work -u 1000 rust cargo build --release
PKG=chamkho-$VERSION-linux-amd64
rm -rf $PKG && mkdir $PKG
cp target/release/wordcut $PKG
cp data/* $PKG
cp LICE* $PKG
tar -cvvzf $PKG.tar.gz $PKG
