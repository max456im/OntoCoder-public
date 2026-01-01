```rust
//! Метаданные формы в реестре — юридико-онтологический паспорт.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RegistryMetadata {
    pub id: String,                   // Уникальный ID (SHA256 от содержимого + jur)
    pub name: String,
    pub version: String,
    pub jurisdiction: String,         // CLA-юрисдикция: CN, BR, ZA, DEFAULT
    pub aenga_compliant: bool,        // Всегда true (иначе не регистрируется)
    pub sgcl_density: f32,            // Фактическая плотность
    pub tags: Vec<String>,            // Например: ["medical", "zodiac", "gaming"]
    pub immutable: bool,              // Нельзя удалить (для научных форм)
    pub registered_at: String,        // ISO 8601
    pub owner: Option<String>,        // Для коммерческих форм
    pub cla_model: String,            // Путь к CLA-шаблону, например: "legal/LICENSE-MODELS/CLA-CN.md"
    pub aenga_hash: String,           // Хеш AENGA-конфигурации
}

impl RegistryMetadata {
    pub fn new_from_form(form: &super::super::compiler::ontoc::parser::OntoForm, jur: &str) -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let id = format!("onto-{}-{}", jur.to_lowercase(), now);

        Self {
            id,
            name: form.meta.name.clone(),
            version: form.meta.version.clone(),
            jurisdiction: jur.to_string(),
            aenga_compliant: true,
            sgcl_density: form.ontology.phases.iter().map(|p| p.density_score).sum::<f32>() / form.ontology.phases.len() as f32,
            tags: vec![], // можно расширить
            immutable: false,
            registered_at: chrono::Utc::now().to_rfc3339(),
            owner: None,
            cla_model: format!("legal/LICENSE-MODELS/CLA-{}.md", jur.to_uppercase()),
            aenga_hash: "a1b2c3d4".to_string(), // в реальности — хеш
        }
    }
}
```