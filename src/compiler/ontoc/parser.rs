```rust
use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OntoForm {
    pub meta: Meta,
    pub ethics: Ethics,
    pub ontology: Ontology,
    pub runtime: Runtime,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Meta {
    pub name: String,
    pub version: String,
    pub cla_jurisdiction: String,
    pub aenga_binding: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ethics {
    pub aenga_core: AENGAConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AENGAConfig {
    pub dignity_preservation: bool,
    pub autonomy_respect: bool,
    pub non_instrumentalization: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ontology {
    pub phases: Vec<Phase>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Phase {
    pub name: String,
    pub density_score: f32,
    pub ethical_constraints: Vec<String>,
    pub social_proximity: Option<f32>,
    pub mbti_alignment: Option<String>,
    pub phase_context: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Runtime {
    pub sgRL_protection: String, // "enabled" | "disabled"
}

pub fn parse_onto_file(path: &Path) -> Result<OntoForm, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(path)?;
    let form: OntoForm = serde_yaml::from_str(&contents)?;
    Ok(form)
}
```