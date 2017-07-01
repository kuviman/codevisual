#!/bin/bash

set -e

rsync -avz --delete examples/public pi@pi.kuviman.com:/home/pi/codevisual
