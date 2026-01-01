```markdown
# Как опубликовать онтологическую форму

> **Пошаговое руководство для исследователей и разработчиков**

---

## Шаг 1: Выберите юрисдикцию

Определите, где будут использоваться ваши данные:

```yaml
meta:
  cla_jurisdiction: "CN"  # ← Обязательно!
```

См. [`legal/JURISDICTION_GUIDE.md`](../legal/JURISDICTION_GUIDE.md)

---

## Шаг 2: Напишите онтоформу

Шаблон:

```yaml
# ontotype: onto-form/v2
meta:
  name: "My Zodiac Advisor"
  version: "1.0.0"
  cla_jurisdiction: "CN"
  aenga_binding: true

ethics:
  aenga_core:
    dignity_preservation: true
    autonomy_respect: true
    non_instrumentalization: true

ontology:
  phases:
    - name: "Horoscope Reading"
      density_score: 0.85  # ← ≥ 0.7
      ethical_constraints:
        - "No financial advice"
        - "No biometric correlation"
      social_proximity: 0.6
      mbti_alignment: "INFJ"

runtime:
  sgRL_protection: enabled
```

---

## Шаг 3: Проверьте соответствие

```bash
# Проверка AENGA
./scripts/verify-aenga.sh my-form.onto

# Проверка плотности
python3 ./scripts/validate-sgcl.py my-form.onto

# Проверка CLA
node ./scripts/check-cla-compliance.js my-form.onto
```

---

## Шаг 4: Скомпилируйте

```bash
ontoc compile my-form.onto --output my-form.ontobin
```

> Если проверки не пройдены — компиляция **аварийно завершится**.

---

## Шаг 5: Опубликуйте

- В GitHub: добавьте в `examples/applied/` + PR  
- В научной работе: цитируйте через `CITATION.cff`  
- В продакшене: укажите в документации:

> «Эта система соответствует OntoCoder Public 2.0 и включает AENGA-ядро. CLA-юрисдикция: CN.»

---
```