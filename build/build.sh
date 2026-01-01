```bash
#!/bin/bash
# build.sh — кроссплатформенная сборка OntoCoder Public без Docker
# Поддерживает: Linux, macOS, Windows (через WSL или MSVC)

set -e

# Параметры по умолчанию
TARGET_OS="${BUILD_OS:-$(uname -s | tr '[:upper:]' '[:lower:]')}"
TARGET_ARCH="${BUILD_ARCH:-$(uname -m)}"
OUTPUT_DIR="./build/artifacts"
RUST_TARGET=""

# Преобразование архитектур
case "$TARGET_ARCH" in
  x86_64|amd64)  RUST_ARCH="x86_64" ;;
  aarch64|arm64) RUST_ARCH="aarch64" ;;
  *) echo "Unsupported architecture: $TARGET_ARCH"; exit 1 ;;
esac

# Определение Rust target triple
case "$TARGET_OS" in
  linux*)
    RUST_TARGET="${RUST_ARCH}-unknown-linux-gnu"
    ;;
  darwin*|macos*)
    RUST_TARGET="${RUST_ARCH}-apple-darwin"
    ;;
  mingw*|msys*|windows*)
    RUST_TARGET="${RUST_ARCH}-pc-windows-msvc"
    ;;
  *)
    echo "Unsupported OS: $TARGET_OS"
    exit 1
    ;;
esac

# Создание директории артефактов
mkdir -p "$OUTPUT_DIR"

# Установка Rust, если отсутствует
if ! command -v rustc &> /dev/null; then
  echo "Rust not found. Installing via rustup..."
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
  source "$HOME/.cargo/env"
fi

# Добавление target, если нужно
rustup target add "$RUST_TARGET" || true

# Сборка компонентов
echo "📦 Building for $RUST_TARGET..."

# 1. ontoc (компилятор)
echo "→ Building ontoc..."
cargo build --release --target "$RUST_TARGET" -p ontoc
cp "target/$RUST_TARGET/release/ontoc"* "$OUTPUT_DIR/"

# 2. onto-runtime (среда выполнения)
echo "→ Building onto-runtime..."
cargo build --release --target "$RUST_TARGET" -p onto-runtime
cp "target/$RUST_TARGET/release/onto-runtime"* "$OUTPUT_DIR/"

# 3. ontoreg (реестр)
echo "→ Building ontoreg..."
cargo build --release --target "$RUST_TARGET" -p ontoreg
cp "target/$RUST_TARGET/release/ontoreg"* "$OUTPUT_DIR/"

# 4. onto-runtime.wasm (для веба)
if command -v wasm-pack &> /dev/null; then
  echo "→ Building onto-runtime.wasm..."
  cd src/runtime/onto-runtime-wasm
  wasm-pack build --release --target web
  cp pkg/onto_runtime_wasm.js pkg/onto_runtime_wasm_bg.wasm "$OUTPUT_DIR/"
  cd ../../../
else
  echo "⚠️  wasm-pack not found. Skipping WebAssembly build."
fi

# Именование артефактов
rename_artifacts() {
  local prefix="ontocoder-public-v2.0.0"
  for file in "$OUTPUT_DIR"/*; do
    if [[ -f "$file" ]]; then
      local name=$(basename "$file")
      local new_name="${prefix}-${TARGET_OS}-${TARGET_ARCH}-${name}"
      mv "$file" "$OUTPUT_DIR/$new_name"
    fi
  done
}

rename_artifacts

echo "✅ Build complete. Artifacts in $OUTPUT_DIR"
ls -l "$OUTPUT_DIR"
```