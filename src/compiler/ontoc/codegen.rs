```rust
use std::fs;
use std::path::Path;
use serde::{Serialize, Deserialize};
use crate::parser::OntoForm;

#[derive(Serialize, Deserialize)]
struct OntoBinary {
    version: String,
    aenga_hash: String,
    sgcl_verified: bool,
    cla_jurisdiction: String,
    payload: Vec<u8>,
}

pub fn generate_ontobin(form: &OntoForm, output_path: &Path) -> Result<(), String> {
    // Простая сериализация (в реальности — бинарный формат с подписью)
    let bin = OntoBinary {
        version: "2.0".to_string(),
        aenga_hash: "a1b2c3d4".to_string(), // в реальности — хеш AENGA-конфига
        sgcl_verified: true,
        cla_jurisdiction: form.meta.cla_jurisdiction.clone(),
        payload: serde_json::to_vec(form).map_err(|e| e.to_string())?,
    };

    let data = serde_json::to_vec(&bin).map_err(|e| e.to_string())?;
    fs::write(output_path, data).map_err(|e| e.to_string())?;

    Ok(())
}
```