```rust
//! Простой индекс для поиска.

use std::fs;
use std::path::PathBuf;
use serde_json;
use crate::metadata::RegistryMetadata;

pub struct FormIndex {
    base_dir: PathBuf,
}

impl FormIndex {
    pub fn new(base_dir: PathBuf) -> Self {
        Self { base_dir }
    }

    pub fn search(&self, jurisdiction: Option<&str>, min_density: Option<f32>) -> Result<Vec<RegistryMetadata>, String> {
        let mut results = Vec::new();
        for entry in fs::read_dir(&self.base_dir).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "json") {
                if let Ok(meta_bytes) = fs::read(&path) {
                    if let Ok(meta) = serde_json::from_slice::<RegistryMetadata>(&meta_bytes) {
                        if let Some(jur) = jurisdiction {
                            if meta.jurisdiction != jur { continue; }
                        }
                        if let Some(d) = min_density {
                            if meta.sgcl_density < d { continue; }
                        }
                        results.push(meta);
                    }
                }
            }
        }
        Ok(results)
    }
}
```