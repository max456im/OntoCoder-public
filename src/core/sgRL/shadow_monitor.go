```go
// Package sgrl implements the Self-Guarding Reasoning Layer.
// It runs in the background and ensures AENGA core integrity.

package sgrl

import (
	"os"
	"runtime"
	"syscall"
	"time"
	"unsafe"
)

// AENGAIntegrityMonitor отслеживает целостность AENGA-ядра.
type AENGAIntegrityMonitor struct {
	aengaBaseAddr uintptr
	pageSize      uintptr
}

// NewAENGAIntegrityMonitor создаёт монитор для указанного адреса AENGA.
func NewAENGAIntegrityMonitor(aengaAddr uintptr) *AENGAIntegrityMonitor {
	pageSize := uintptr(os.Getpagesize())
	return &AENGAIntegrityMonitor{
		aengaBaseAddr: aengaAddr,
		pageSize:      pageSize,
	}
}

// checkMemoryPermissions проверяет, не изменены ли права доступа к памяти AENGA.
func (m *AENGAIntegrityMonitor) checkMemoryPermissions() {
	// Пытаемся установить стандартные права (чтение + выполнение)
	_, err := syscall.Mprotect(
		unsafe.Pointer(m.aengaBaseAddr),
		m.pageSize,
		syscall.PROT_READ|syscall.PROT_EXEC,
	)
	if err != nil {
		panic("SGRL: Memory protection violation — AENGA core tampered")
	}
}

// checkEnvironment сканирует окружение на признаки обхода.
func (m *AENGAIntegrityMonitor) checkEnvironment() {
	// Проверка LD_PRELOAD (Linux)
	if ldPreload := os.Getenv("LD_PRELOAD"); ldPreload != "" {
		panic("SGRL: LD_PRELOAD detected — potential AENGA bypass")
	}

	// Проверка DYLD_INSERT_LIBRARIES (macOS)
	if dyld := os.Getenv("DYLD_INSERT_LIBRARIES"); dyld != "" {
		panic("SGRL: DYLD_INSERT_LIBRARIES detected")
	}

	// Проверка отладчика (Linux/macOS)
	if isDebuggerAttached() {
		panic("SGRL: Debugger attached — runtime integrity compromised")
	}
}

// isDebuggerAttached определяет, подключён ли отладчик.
func isDebuggerAttached() bool {
	// Простая эвристика: проверка времени выполнения
	start := time.Now()
	time.Sleep(1 * time.Microsecond)
	return time.Since(start) > 10*time.Millisecond
}

// Start запускает непрерывный мониторинг в отдельной горутине.
func (m *AENGAIntegrityMonitor) Start() {
	go func() {
		ticker := time.NewTicker(50 * time.Millisecond) // 20 Гц
		defer ticker.Stop()

		for range ticker.C {
			m.checkMemoryPermissions()
			m.checkEnvironment()
		}
	}()
}

// ForceIntegrityCheck — синхронная проверка (для критических операций)
func (m *AENGAIntegrityMonitor) ForceIntegrityCheck() {
	m.checkMemoryPermissions()
	m.checkEnvironment()
}
```
