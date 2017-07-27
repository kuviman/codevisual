#!/bin/bash
set -e
target=asmjs
if [ -n "$1" ]; then
    target=$1
fi
./make-web.sh $target tutorial-01
./make-web.sh $target tutorial-02
./make-web.sh $target tutorial-03