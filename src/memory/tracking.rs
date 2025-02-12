// src/memory/tracking.rs
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Default)]
pub struct MemoryStats {
    total_allocated: AtomicUsize,
    active_blocks: AtomicUsize,
    total_blocks_created: AtomicUsize,
    peak_usage: AtomicUsize,
    allocation_count: AtomicUsize,
}

impl MemoryStats {
    pub fn record_allocation(&self, size: usize) {
        let current = self.total_allocated.fetch_add(size, Ordering::Relaxed);
        let new_total = current + size;
        
        // Update peak if necessary
        let mut peak = self.peak_usage.load(Ordering::Relaxed);
        while new_total > peak {
            match self.peak_usage.compare_exchange_weak(
                peak,
                new_total,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(x) => peak = x,
            }
        }
        
        self.allocation_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_block_creation(&self) {
        self.active_blocks.fetch_add(1, Ordering::Relaxed);
        self.total_blocks_created.fetch_add(1, Ordering::Relaxed);
    }
}