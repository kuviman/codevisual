#!/bin/bash

set -e

config=release
if [ -n "$1" ]; then
    config=$1
fi

target=wasm32 #asmjs
cargoTarget=$target-unknown-emscripten
cargoArgs=
if [ "$config" == "release" ]; then
    cargoArgs=--release
fi

export EMSCRIPTEN=`python -c "execfile('$HOME/.emscripten'); print(EMSCRIPTEN_ROOT)"`
PATH="$EMSCRIPTEN:$PATH"

cargo build $cargoArgs --target=$cargoTarget --example playground
cp target/$cargoTarget/$config/examples/playground.js examples/public/code.js
if [ "target" == "wasm" ]; then
    cp target/$cargoTarget/$config/examples/*.wasm examples/public
fi