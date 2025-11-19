#!/bin/bash
# WebAssembly バインドをビルドするスクリプト
# デフォルトでは web ターゲットでビルドします

set -e

TARGET="${1:-web}"

cd "$(dirname "$0")/wasm"
wasm-pack build --target "$TARGET"

echo ""
echo "✅ WASM パッケージが wasm/pkg ディレクトリに生成されました"

