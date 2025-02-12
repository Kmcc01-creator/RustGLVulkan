// src/graphics/buffer.rs
pub struct BufferManager<T: GraphicsMemoryManager> {
    memory_manager: T,
    buffer_allocations: HashMap<T::BufferHandle, BufferAllocation>,
    staging_pool: StagingPool,
}
