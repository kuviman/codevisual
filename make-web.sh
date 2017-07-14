#!/bin/bash

set -e

config=release
target=asmjs
if [ -n "$1" ]; then
    target=$1
fi
cargoTarget=$target-unknown-emscripten
cargoArgs=
if [ "$config" == "release" ]; then
    cargoArgs=--release
fi

export EMSCRIPTEN=`python -c "execfile('$HOME/.emscripten'); print(EMSCRIPTEN_ROOT)"`
PATH="$EMSCRIPTEN:$PATH"

export EMMAKEN_CFLAGS="-s TOTAL_MEMORY=$(expr 256 "*" 1024 "*" 1024)"

name=playground
target_dir=target/web/$name
cargo build $cargoArgs --target=$cargoTarget --example $name
mkdir -p $target_dir
cp -r examples/$name/static/* $target_dir
cp target/$cargoTarget/$config/examples/$name.js $target_dir/code.js
if [ "$target" == "wasm32" ]; then
    cp target/$cargoTarget/$config/examples/*.wasm $target_dir
fi