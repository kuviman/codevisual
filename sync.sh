#!/bin/bash

set -e

./make-web.sh asmjs
rsync -avz --delete playground/public pi@pi.kuviman.com:/home/pi/codevisual-playground/asmjs

./make-web.sh wasm32
rsync -avz --delete playground/public pi@pi.kuviman.com:/home/pi/codevisual-playground/wasm
