// src/memory/pool.rs
pub struct MemoryPool<T: MemoryManager> {
    allocator: T,
    blocks: Vec<MemoryBlock>,
    free_blocks: BTreeMap<usize, Vec<usize>>, // Size -> Block indices
    debug_info: Option<DebugInfo>,
}

