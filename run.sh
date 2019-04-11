#!/bin/bash
set -e
target=release
cfg=""
cargo build --$target --target=armv7-unknown-linux-gnueabihf $@  || exit 0
cp target/armv7-unknown-linux-gnueabihf/$target/plato dist/ && \
arm-poky-linux-gnueabi-strip dist/plato
file dist/plato 
ssh rm 'kill $(pidof plato)' ||true
scp dist/plato rm:~/dist
ssh rm "~/dist/run | tee output"
