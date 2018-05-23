#!/bin/bash

set -euo pipefail

cargo web test --package codevisual
cargo web test --package codevisual-demo