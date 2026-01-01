```rust
//! Валидатор SGCL для интеграции в компилятор и runtime.

use crate::onto::form::OntoForm;
use crate::sgCL::density::validate_sgcl_compliance;

pub struct SGCLValidator;

impl SGCLValidator {
    /// Валидация на этапе компиляции.
    pub fn compile_time_check(form: &OntoForm) -> Result<(), String> {
        validate_sgcl_compliance(form)
    }

    /// Валидация на этапе выполнения (например, при динамической загрузке формы).
    pub fn runtime_check(form: &OntoForm) -> Result<(), String> {
        validate_sgcl_compliance(form)
    }

    /// Быстрая проверка: только плотность, без полной валидации.
    pub fn quick_density_check(form: &OntoForm) -> f32 {
        crate::sgCL::density::compute_ontological_density(form)
    }
}
```