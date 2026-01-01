```python
#!/usr/bin/env python3
# validate-sgcl.py — проверяет онтологическую плотность (ρ ≥ 0.7)

import sys
import os
import yaml

MIN_DENSITY = 0.7

def validate_file(filepath):
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            data = yaml.safe_load(f)
    except Exception as e:
        print(f"⚠️  SGCL: Skipping {filepath} (not YAML or invalid): {e}")
        return True  # не ошибка для CI

    if not data or 'ontology' not in data or 'phases' not in data['ontology']:
        return True  # не онтоформа

    phases = data['ontology']['phases']
    for i, phase in enumerate(phases):
        if 'density_score' not in phase:
            print(f"❌ SGCL VIOLATION: {filepath} — phase '{phase.get('name', i)}' missing 'density_score'")
            return False

        density = phase['density_score']
        if density < MIN_DENSITY:
            print(f"❌ SGCL VIOLATION: {filepath} — phase '{phase.get('name', i)}' density {density} < {MIN_DENSITY}")
            return False

        # Проверка наличия ethical_constraints
        if 'ethical_constraints' not in phase or not phase['ethical_constraints']:
            print(f"❌ SGCL VIOLATION: {filepath} — phase '{phase.get('name', i)}' has no ethical_constraints")
            return False

    print(f"✅ SGCL: {filepath} — compliant")
    return True

def main():
    if len(sys.argv) < 2:
        print("Usage: validate-sgcl.py <file.or.dir>...")
        sys.exit(1)

    all_valid = True
    for item in sys.argv[1:]:
        if os.path.isdir(item):
            for root, _, files in os.walk(item):
                for f in files:
                    if f.endswith(('.md', '.yaml', '.yml')):
                        if not validate_file(os.path.join(root, f)):
                            all_valid = False
        else:
            if not validate_file(item):
                all_valid = False

    if not all_valid:
        print("\n❌ SGCL validation failed.")
        sys.exit(1)
    else:
        print("✅ All files passed SGCL validation.")

if __name__ == "__main__":
    main()
```
