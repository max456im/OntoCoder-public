```rust
//! Этическая паника — неустранимая остановка системы при нарушении AENGA.

use std::process;
use std::sync::atomic::{AtomicBool, Ordering};

static PANIC_LOCK: AtomicBool = AtomicBool::new(false);

/// Генерирует неустранимую ошибку и завершает процесс.
/// Код выхода 13 — зарезервирован для этических нарушений.
pub fn ethical_violation(code: &str, message: &str) -> ! {
    // Предотвращает повторный вызов в рекурсии
    if PANIC_LOCK.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
        eprintln!("\x1b[91m[AENGA PANIC] {} — {}\x1b[0m", code, message);
        eprintln!("\x1b[93m[SYSTEM] Ethical integrity lost. Halting execution.\x1b[0m");

        // В продакшене: запись в защищённый журнал (например, append-only ledger)
        log_to_immutable_store(code, message);

        // Выход с кодом 13 — стандарт для AENGA-нарушений
        process::exit(13);
    } else {
        // Уже в процессе паники — принудительный exit
        process::exit(13);
    }
}

/// Заглушка для журналирования. В реальной системе — интеграция с ledger.
fn log_to_immutable_store(code: &str, message: &str) {
    // Пример: запись в stderr или безопасный логгер
    eprintln!("[ETHICAL_LOG] {} | {}", code, message);
    // В продакшене: отправка в распределённый журнал с хешированием
}
```