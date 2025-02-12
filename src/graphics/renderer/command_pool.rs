// src/graphics/renderer/command_pool.rs

use crate::graphics::{
    core::{GraphicsBackend, GraphicsError},
    command::CommandBuffer,
};

pub struct CommandPool<B: GraphicsBackend> {
    // For a minimal implementation, we don't need to pool buffers yet.
    // We can add pooling logic later.
    _backend_marker: std::marker::PhantomData<B>, // To hold the backend type
}

impl<B: GraphicsBackend> CommandPool<B> {
    pub fn new() -> Result<Self, GraphicsError> {
        Ok(Self {
            _backend_marker: std::marker::PhantomData,
        })
    }

    pub fn get_current_buffer(&mut self, backend: &B) -> Result<B::CommandBuffer, GraphicsError> {
        // Minimal implementation: Create a *new* command buffer every time.
        // In a real pool, you'd reuse command buffers.
        backend.create_command_buffer()
    }

    // If you were implementing proper pooling, you would have methods to:
    // - Reset/reuse command buffers
    // - Manage a pool of available command buffers
}