#!/bin/bash

set -euo pipefail

cargo test --package codevisual
cargo test --package codevisual-demo