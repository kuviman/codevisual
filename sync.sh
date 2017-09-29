#!/bin/bash

set -e

./make-web.sh asmjs
./make-web.sh wasm32
./make-tutorial.sh asmjs
./make-tutorial.sh wasm32
# ./make-windows.sh
rsync -avz --delete target/web/* pi@pi.kuviman.com:/home/pi/codevisual/
