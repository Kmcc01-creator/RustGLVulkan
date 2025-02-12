// Resource management
// src/graphics/resources/mod.rs
pub trait ResourceManager {
    type Buffer;
    type Texture;
    type Shader;
    type Pipeline;
    
    fn create_buffer(&mut self, desc: &BufferDesc) -> Result<Self::Buffer, GraphicsError>;
    fn create_texture(&mut self, desc: &TextureDesc) -> Result<Self::Texture, GraphicsError>;
    fn create_shader(&mut self, desc: &ShaderDesc) -> Result<Self::Shader, GraphicsError>;
    fn create_pipeline(&mut self, desc: &PipelineDesc) -> Result<Self::Pipeline, GraphicsError>;
    
    fn destroy_buffer(&mut self, buffer: Self::Buffer);
    fn destroy_texture(&mut self, texture: Self::Texture);
    fn destroy_shader(&mut self, shader: Self::Shader);
    fn destroy_pipeline(&mut self, pipeline: Self::Pipeline);
}
