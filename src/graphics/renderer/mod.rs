
// Backend-agnostic renderer
// src/graphics/renderer/mod.rs
pub struct Renderer<B: GraphicsBackend> {
    backend: B,
    resource_manager: Box<dyn ResourceManager>,
    command_pool: CommandPool<B>,
    current_frame: usize,
}

impl<B: GraphicsBackend> Renderer<B> {
    pub fn new(backend: B) -> Result<Self, GraphicsError> {
        // Initialize renderer with specific backend
        Ok(Self {
            backend,
            resource_manager: Box::new(DefaultResourceManager::new()),
            command_pool: CommandPool::new()?,
            current_frame: 0,
        })
    }
    
    pub fn begin_frame(&mut self) -> Result<(), GraphicsError> {
        let command_buffer = self.command_pool.get_current_buffer()?;
        command_buffer.begin()?;
        Ok(())
    }
    
    pub fn end_frame(&mut self) -> Result<(), GraphicsError> {
        let command_buffer = self.command_pool.get_current_buffer()?;
        command_buffer.end()?;
        self.current_frame += 1;
        Ok(())
    }
}