// src/debug/mod.rs
pub mod memory;
pub mod profiler;
pub mod trace;

// Export common debug utilities
pub use self::memory::MemoryDebugger;
pub use self::trace::TracingFacility;

// Common debug configuration
#[derive(Debug, Clone)]
pub struct DebugConfig {
    pub enabled: bool,
    pub trace_allocations: bool,
    pub profile_performance: bool,
    pub log_level: LogLevel,
}

#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    Verbose,
    Normal,
    Critical,
}