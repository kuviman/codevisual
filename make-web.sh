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
if [ -n "$2" ]; then
    name=$2
fi
target_dir=target/web/$target/$name
cargo build $cargoArgs --target=$cargoTarget --example $name
mkdir -p $target_dir
cp target/$cargoTarget/$config/build/codevisual_core_html-*/out/lib.html $target_dir/codevisual.html
cp target/$cargoTarget/$config/build/codevisual_core_css-*/out/lib.css $target_dir/codevisual.css
cp target/$cargoTarget/$config/build/codevisual_core_js-*/out/lib.js $target_dir/codevisual.js
cp src/core/web/favicon.ico $target_dir
cp -r src/core/web/lib $target_dir
if [ "$name" == "playground" ]; then
    cp -r examples/$name/static/* $target_dir
else
    mv $target_dir/codevisual.html $target_dir/index.html
fi
cp target/$cargoTarget/$config/examples/$name.js $target_dir/code.js
if [ "$target" == "wasm32" ]; then
    cp target/$cargoTarget/$config/examples/*.wasm $target_dir
fi