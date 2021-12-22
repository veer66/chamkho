#!/bin/bash

VERSION=1.0.3
docker run -it --rm -v $(pwd):/work -w /work -u 1000 rust cargo build --release
rm -rf chamkho-$VERSION-amd64 && mkdir chamkho-$VERSION-amd64
cp target/release/wordcut chamkho-$VERSION-amd64
cp data/* chamkho-$VERSION-amd64
tar -cvvzf chamkho-$VERSION-amd64.tar.gz chamkho-$VERSION-amd64
