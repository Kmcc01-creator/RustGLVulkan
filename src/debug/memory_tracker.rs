// src/debug/memory_tracker.rs
pub struct MemoryTracker {
    allocations: HashMap<*mut u8, AllocationInfo>,
    statistics: MemoryStats,
    traces: Vec<MemoryTrace>,
    config: DebugConfig,
}

#[derive(Debug)]
pub struct AllocationInfo {
    size: usize,
    alignment: usize,
    memory_type: MemoryType,
    stack_trace: Option<Backtrace>,
    timestamp: Instant,
}

// Debug wrapper for memory managers
pub struct DebugMemoryManager<T: MemoryManager> {
    inner: T,
    tracker: Arc<MemoryTracker>,
}