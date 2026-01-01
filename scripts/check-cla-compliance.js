```javascript
#!/usr/bin/env node
// check-cla-compliance.js — проверяет CLA-юрисдикцию и sgRL_protection

const fs = require('fs');
const path = require('path');
const yaml = require('js-yaml');

const SUPPORTED_JURISDICTIONS = ['CN', 'BR', 'ZA', 'DEFAULT'];

function validateFile(filepath) {
    try {
        const content = fs.readFileSync(filepath, 'utf8');
        const data = yaml.load(content);

        if (!data?.meta?.cla_jurisdiction) {
            console.log(`⚠️  CLA: Skipping ${filepath} (no CLA jurisdiction)`);
            return true;
        }

        const jur = data.meta.cla_jurisdiction;
        if (!SUPPORTED_JURISDICTIONS.includes(jur)) {
            console.error(`❌ CLA VIOLATION: ${filepath} — unsupported jurisdiction: ${jur}`);
            return false;
        }

        if (data.runtime?.sgRL_protection !== 'enabled') {
            console.error(`❌ CLA/SGRL VIOLATION: ${filepath} — sgRL_protection must be 'enabled'`);
            return false;
        }

        console.log(`✅ CLA: ${filepath} — jurisdiction ${jur} accepted`);
        return true;
    } catch (e) {
        console.log(`⚠️  CLA: Skipping ${filepath} (not YAML or invalid): ${e.message}`);
        return true;
    }
}

function walkDir(dir, callback) {
    fs.readdirSync(dir).forEach(f => {
        const dirPath = path.join(dir, f);
        const isDirectory = fs.statSync(dirPath).isDirectory();
        isDirectory ? walkDir(dirPath, callback) : callback(dirPath);
    });
}

async function main() {
    const args = process.argv.slice(2);
    if (args.length === 0) {
        console.error("Usage: check-cla-compliance.js <file.or.dir>...");
        process.exit(1);
    }

    let allValid = true;
    for (const item of args) {
        if (fs.statSync(item).isDirectory()) {
            walkDir(item, file => {
                if (/\.(md|yaml|yml)$/.test(file)) {
                    if (!validateFile(file)) allValid = false;
                }
            });
        } else {
            if (!validateFile(item)) allValid = false;
        }
    }

    if (!allValid) {
        console.error("\n❌ CLA compliance check failed.");
        process.exit(1);
    } else {
        console.log("✅ All files passed CLA compliance check.");
    }
}

// 💡 **Зависимость**: `js-yaml`. Установка:  
// ```bash
// npm install -g js-yaml
// # или локально:
// npm init -y && npm install js-yaml
// ```

main();
```
