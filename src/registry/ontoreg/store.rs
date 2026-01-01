```rust
//! Хранилище форм на диске с метаданными.

use std::fs;
use std::path::{Path, PathBuf};
use serde_json;
use crate::metadata::RegistryMetadata;
use crate::super::super::compiler::ontoc::parser::OntoForm;

pub struct FormStore {
    base_dir: PathBuf,
}

impl FormStore {
    pub fn new(base: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let dir = PathBuf::from(base);
        fs::create_dir_all(&dir)?;
        Ok(Self { base_dir: dir })
    }

    pub fn register_form(&mut self, form_path: &Path, jurisdiction: &str) -> Result<(), String> {
        let form_bytes = fs::read(form_path).map_err(|e| e.to_string())?;
        let form: OntoForm = serde_json::from_slice(&form_bytes).map_err(|e| e.to_string())?;

        let meta = crate::validator::validate_form_for_registry(&form, jurisdiction)?;

        // Сохраняем метаданные
        let meta_path = self.base_dir.join(format!("{}.meta.json", meta.id));
        fs::write(meta_path, serde_json::to_vec(&meta).unwrap()).map_err(|e| e.to_string())?;

        // Сохраняем саму форму
        let form_out_path = self.base_dir.join(format!("{}.ontobin", meta.id));
        fs::write(form_out_path, form_bytes).map_err(|e| e.to_string())?;

        Ok(())
    }

    pub fn export_form(&self, id: &str) -> Result<String, String> {
        let meta_path = self.base_dir.join(format!("{}.meta.json", id));
        let meta: RegistryMetadata = serde_json::from_slice(&fs::read(meta_path).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;

        Ok(serde_json::to_string_pretty(&meta).unwrap())
    }
}
```