#!/bin/bash

set -e

config=release
if [ -n "$3" ]; then
    config=$3
fi

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

if [ -n "$2" ]; then
    name=$2
fi
target_dir=target/web/${target}/codewars2017
cargo build ${cargoArgs} --target=${cargoTarget}
cd ..

mkdir -p $target_dir
cp target/${cargoTarget}/${config}/build/codevisual_core_html-*/out/lib.html ${target_dir}/codevisual.html
cp target/${cargoTarget}/${config}/build/codevisual_core_css-*/out/lib.css ${target_dir}/codevisual.css
cp target/${cargoTarget}/${config}/build/codevisual_core_js-*/out/lib.js ${target_dir}/codevisual.js
cp src/core/web/favicon.ico ${target_dir}
cp -r src/core/web/lib ${target_dir}
cp -r codewars2017/static/* ${target_dir}
cp target/${cargoTarget}/${config}/codewars2017.js ${target_dir}/code.js