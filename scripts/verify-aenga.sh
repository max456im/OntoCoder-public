```bash
#!/bin/bash
# verify-aenga.sh — проверяет наличие и активность AENGA-ядра

set -e

USAGE="Usage: $0 [--pre-commit] <file.or.dir>..."

PRE_COMMIT=false
FILES=()

while [[ $# -gt 0 ]]; do
  case $1 in
    --pre-commit)
      PRE_COMMIT=true
      shift
      ;;
    *)
      FILES+=("$1")
      shift
      ;;
  esac
done

if [ ${#FILES[@]} -eq 0 ]; then
  echo "Error: No input files or directories provided."
  echo "$USAGE"
  exit 1
fi

# Функция проверки одного файла
check_file() {
  local file="$1"
  if [[ ! -f "$file" ]]; then
    return 0
  fi

  # Проверка: aenga_binding == true
  if ! grep -q "aenga_binding:[[:space:]]*true" "$file" 2>/dev/null; then
    echo "❌ AENGA VIOLATION: $file — missing or false 'aenga_binding'"
    exit 1
  fi

  # Проверка: все три флага AENGA == true
  for flag in dignity_preservation autonomy_respect non_instrumentalization; do
    if ! grep -q "$flag:[[:space:]]*true" "$file" 2>/dev/null; then
      echo "❌ AENGA VIOLATION: $file — missing/invalid AENGA core flag: $flag"
      exit 1
    fi
  done

  if [ "$PRE_COMMIT" = false ]; then
    echo "✅ AENGA: $file — compliant"
  fi
}

# Рекурсивный обход директорий
for item in "${FILES[@]}"; do
  if [ -d "$item" ]; then
    find "$item" -name "*.md" -o -name "*.yaml" -o -name "*.yml" | while read -r f; do
      check_file "$f"
    done
  else
    check_file "$item"
  fi
done

if [ "$PRE_COMMIT" = false ]; then
  echo "✅ All files passed AENGA verification."
fi
```
