#!/bin/bash

set -e

./make-web.sh asmjs
rsync -avz --delete examples/public pi@pi.kuviman.com:/home/pi/codevisual-asmjs

./make-web.sh wasm32
rsync -avz --delete examples/public pi@pi.kuviman.com:/home/pi/codevisual-wasm
