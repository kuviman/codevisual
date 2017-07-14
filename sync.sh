#!/bin/bash

set -e

./make-web.sh asmjs
rsync -avz --delete target/web/* pi@pi.kuviman.com:/home/pi/codevisual/asmjs

./make-web.sh wasm32
rsync -avz --delete target/web/* pi@pi.kuviman.com:/home/pi/codevisual/wasm
