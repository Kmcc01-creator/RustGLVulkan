// OpenGL implementation
// src/graphics/backends/opengl/mod.rs
pub struct OpenGLBackend {
    context: GLContext,
    state_cache: GLStateCache,
    resource_tracker: ResourceTracker,
}

impl GraphicsBackend for OpenGLBackend {
    type Buffer = GLBuffer;
    type Shader = GLShader;
    type Pipeline = GLPipeline;
    type CommandBuffer = GLCommandBuffer;
    
    fn create_buffer(&self, desc: BufferDesc) -> Result<GLBuffer, GraphicsError> {
        let mut buffer = GLBuffer::new(&self.context);
        buffer.allocate(desc.size, desc.usage)?;
        Ok(buffer)
    }
    
    fn create_shader(&self, desc: ShaderDesc) -> Result<GLShader, GraphicsError> {
        let shader = GLShader::new(&self.context, desc.stage)?;
        shader.compile(desc.source)?;
        Ok(shader)
    }
    
    fn create_pipeline(&self, desc: PipelineDesc) -> Result<GLPipeline, GraphicsError> {
        let pipeline = GLPipeline::new(&self.context);
        pipeline.set_vertex_layout(&desc.vertex_layout)?;
        pipeline.set_shader_stages(&desc.shader_stages)?;
        pipeline.set_rasterizer_state(&desc.rasterizer_state)?;
        pipeline.set_blend_state(&desc.blend_state)?;
        pipeline.set_depth_stencil_state(&desc.depth_stencil_state)?;
        Ok(pipeline)
    }
}