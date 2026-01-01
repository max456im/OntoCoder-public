```rust
use crate::parser::OntoForm;

const MIN_DENSITY: f32 = 0.7;

pub fn validate_sgcl(form: &OntoForm) -> Result<(), String> {
    for phase in &form.ontology.phases {
        if phase.density_score < MIN_DENSITY {
            return Err(format!(
                "SGCL violation: phase '{}' has density {} < {}",
                phase.name, phase.density_score, MIN_DENSITY
            ));
        }
    }

    // Дополнительно: проверка наличия ethical_constraints
    for phase in &form.ontology.phases {
        if phase.ethical_constraints.is_empty() {
            return Err(format!(
                "SGCL violation: phase '{}' has no ethical constraints",
                phase.name
            ));
        }
    }

    println!("[ontoc/SGCL] ✓ Ontological density verified");
    Ok(())
}
```