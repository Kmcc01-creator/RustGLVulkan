// src/graphics/core/mod.rs

/// Trait defining the core graphics backend interface
pub trait GraphicsBackend: Send + Sync {
    type Buffer;
    type Shader;
    type Pipeline;
    type CommandBuffer;
    
    fn create_buffer(&self, desc: BufferDesc) -> Result<Self::Buffer, GraphicsError>;
    fn create_shader(&self, desc: ShaderDesc) -> Result<Self::Shader, GraphicsError>;
    fn create_pipeline(&self, desc: PipelineDesc) -> Result<Self::Pipeline, GraphicsError>;
    fn create_command_buffer(&self) -> Result<Self::CommandBuffer, GraphicsError>;
}

// Core types shared across all backends
#[derive(Debug, Clone)]
pub struct BufferDesc {
    pub size: usize,
    pub usage: BufferUsage,
    pub memory_type: MemoryType,
}

#[derive(Debug, Clone)]
pub struct ShaderDesc {
    pub stage: ShaderStage,
    pub source: ShaderSource,
    pub entry_point: String,
}

#[derive(Debug, Clone)]
pub struct PipelineDesc {
    pub vertex_layout: VertexLayout,
    pub shader_stages: Vec<ShaderStageDesc>,
    pub rasterizer_state: RasterizerState,
    pub blend_state: BlendState,
    pub depth_stencil_state: DepthStencilState,
}