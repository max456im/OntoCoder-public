```rust
//! Модуль вычисления онтологической плотности (ρ)

use crate::onto::form::{OntoNode, OntoForm};

/// Минимально допустимая плотность по SGCL.
pub const MIN_SGCL_DENSITY: f32 = 0.7;

/// Вычисляет онтологическую плотность онтоформы.
pub fn compute_ontological_density(form: &OntoForm) -> f32 {
    let nodes: Vec<&OntoNode> = form
        .ontology
        .phases
        .iter()
        .map(|p| &p.node)
        .collect();

    if nodes.is_empty() {
        return 0.0;
    }

    let total_nodes = nodes.len() as f32;
    let mut total_weight = 0.0;

    for node in nodes {
        // Инварианты: +1 за каждый
        total_weight += node.invariants.len() as f32;

        // Этические ограничения: +1 за каждое
        total_weight += node.ethical_constraints.len() as f32;

        // Фазовый контекст: +1, если указан
        if node.phase_context.is_some() {
            total_weight += 1.0;
        }

        // Социальная привязка: +1, если указана
        if node.social_proximity.is_some() || node.mbti_alignment.is_some() {
            total_weight += 1.0;
        }
    }

    total_weight / total_nodes
}

/// Проверяет, соответствует ли форма требованиям SGCL.
pub fn validate_sgcl_compliance(form: &OntoForm) -> Result<(), String> {
    let density = compute_ontological_density(form);
    if density < MIN_SGCL_DENSITY {
        Err(format!(
            "SGCL violation: ontological density {} < {}",
            density, MIN_SGCL_DENSITY
        ))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_high_density_form() {
        // Имитация формы с богатым контекстом
        let form = mock_ontoform_with_density(0.85);
        assert!(validate_sgcl_compliance(&form).is_ok());
    }

    #[test]
    fn test_low_density_form() {
        let form = mock_ontoform_with_density(0.3);
        assert!(validate_sgcl_compliance(&form).is_err());
    }
}
```