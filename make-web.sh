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

cargo build $cargoArgs --target=$cargoTarget --example playground
cp target/$cargoTarget/$config/examples/playground.js playground/public/code.js
if [ "$target" == "wasm32" ]; then
    cp target/$cargoTarget/$config/examples/*.wasm playground/public
fi