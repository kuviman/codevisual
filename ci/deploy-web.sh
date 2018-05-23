#!/bin/bash

set -euo pipefail

cargo web deploy --target wasm32-unknown-emscripten --release --package codevisual-demo
cargo doc --target wasm32-unknown-emscripten --release --package codevisual
cp target/doc target/deploy
echo '<meta http-equiv=refresh content=0;url=codevisual/index.html>' > target/deploy/doc/index.html