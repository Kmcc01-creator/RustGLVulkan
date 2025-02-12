
// Pipeline state management
// src/graphics/pipeline/state.rs
#[derive(Debug, Clone)]
pub struct PipelineState {
    vertex_layout: VertexLayout,
    rasterizer_state: RasterizerState,
    blend_state: BlendState,
    depth_stencil_state: DepthStencilState,
    viewport: Viewport,
    scissor: ScissorRect,
}

impl PipelineState {
    pub fn builder() -> PipelineStateBuilder {
        PipelineStateBuilder::new()
    }
    
    pub fn apply<B: GraphicsBackend>(&self, backend: &B) -> Result<(), GraphicsError> {
        // Apply state changes through backend-specific calls
        Ok(())
    }
}
