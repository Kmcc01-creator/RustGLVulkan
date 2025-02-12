// src/graphics/texture.rs
pub struct TextureManager<T: GraphicsMemoryManager> {
    memory_manager: T,
    texture_allocations: HashMap<T::TextureHandle, TextureAllocation>,
    mip_generator: MipGenerator,
}