// src/memory/cache.rs
pub struct ResourceCache<T: GraphicsMemoryManager> {
    memory_manager: T,
    cached_buffers: HashMap<BufferDesc, WeakHandle<T::BufferHandle>>,
    cached_textures: HashMap<TextureDesc, WeakHandle<T::TextureHandle>>,
    statistics: CacheStats,
}