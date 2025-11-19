#!/bin/bash
# WebAssembly bindings テストを実行するスクリプト

set -e

cd "$(dirname "$0")/wasm"
wasm-pack test --node

