#!/bin/bash

config=release
if [ -n "$1" ]; then
    config=$1
fi

cargoTarget=asmjs-unknown-emscripten
cargoArgs=
if [ "$config" == "release" ]; then
    cargoArgs=--release
fi

export EMSCRIPTEN=`python -c "execfile('$HOME/.emscripten'); print(EMSCRIPTEN_ROOT)"`
PATH="$EMSCRIPTEN:$PATH"

cargo build $cargoArgs --target=$cargoTarget --example playground
cp target/$cargoTarget/$config/examples/playground.js examples/public/code.js