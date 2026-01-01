```bash
#!/bin/bash
# generate-citation.sh — генерирует CITATION.cff из метаданных релиза

set -e

if [ ! -f "CITATION.cff.template" ]; then
  echo "Error: CITATION.cff.template not found."
  exit 1
fi

# Получаем версию из последнего git tag
VERSION=$(git describe --tags --abbrev=0 2>/dev/null || echo "0.0.0")

# Получаем дату последнего коммита
DATE=$(git log -1 --format=%ad --date=short)

# Заменяем плейсхолдеры
sed -e "s/{{VERSION}}/$VERSION/g" \
    -e "s/{{DATE_RELEASED}}/$DATE/g" \
    CITATION.cff.template > CITATION.cff

# 📝 **Требуется файл `CITATION.cff.template`** в корне (не включён в `scripts/`).

### 🔐 Права на выполнение

# Убедитесь, что все скрипты исполняемые:

# ```bash
# chmod +x scripts/*.sh scripts/*.py scripts/*.js
# ```

# ✅ Интеграция

# - **pre-commit**: уже настроен в `.pre-commit-config.yaml`,  
# - **CI**: вызывается в `.github/workflows/compliance-check.yml`,  
# - **CLI**: можно запускать вручную:
#  ```bash
#  ./scripts/verify-aenga.sh examples/
#  ./scripts/validate-sgcl.py examples/
#  ./scripts/check-cla-compliance.js examples/
#  ```

echo "✅ Generated CITATION.cff for version $VERSION ($DATE)"
```
