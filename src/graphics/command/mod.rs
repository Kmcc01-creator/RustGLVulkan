
// Command buffer abstraction
// src/graphics/command/mod.rs
pub trait CommandBuffer {
    fn begin(&mut self) -> Result<(), GraphicsError>;
    fn end(&mut self) -> Result<(), GraphicsError>;
    fn bind_pipeline(&mut self, pipeline: &impl Pipeline) -> Result<(), GraphicsError>;
    fn bind_vertex_buffer(&mut self, buffer: &impl Buffer, offset: u64) -> Result<(), GraphicsError>;
    fn draw(&mut self, vertex_count: u32, instance_count: u32) -> Result<(), GraphicsError>;
    fn draw_indexed(&mut self, index_count: u32, instance_count: u32) -> Result<(), GraphicsError>;
}
