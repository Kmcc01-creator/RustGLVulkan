// src/debug/trace.rs
use std::sync::atomic::{AtomicU64, Ordering};
use parking_lot::RwLock;

pub struct TracingFacility {
    enabled: bool,
    current_frame: AtomicU64,
    traces: RwLock<Vec<DebugTrace>>,
}

#[derive(Debug)]
struct DebugTrace {
    frame: u64,
    category: TraceCategory,
    message: String,
    timestamp: std::time::SystemTime,
}

#[derive(Debug)]
enum TraceCategory {
    Memory,
    Graphics,
    Scene,
    Performance,
}