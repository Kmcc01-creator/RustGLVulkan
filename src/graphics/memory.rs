// src/graphics/memory.rs
pub enum GraphicsAPI {
    OpenGL,
    Vulkan,
}

pub struct GraphicsMemoryConfig {
    api: GraphicsAPI,
    device_local_size: usize,
    host_visible_size: usize,
    staging_buffer_size: usize,
}

// Graphics memory types
#[derive(Debug, Clone, Copy)]
pub enum MemoryType {
    DeviceLocal,    // GPU-only access
    HostVisible,    // CPU-visible
    HostCoherent,   // No explicit flush needed
    HostCached,     // Cached memory access
}

// Memory allocation descriptor
pub struct MemoryDesc {
    size: usize,
    alignment: usize,
    memory_type: MemoryType,
    usage: MemoryUsage,
}