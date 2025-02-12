// src/graphics/backends/opengl/buffer.rs
pub struct GLBuffer {
    gl: Arc<gl::Gl>,
    handle: gl::types::GLuint,
    size: usize,
    usage: BufferUsage,
    target: gl::types::GLenum,
    stride: usize,
    attributes: Vec<VertexAttribute>,
}

impl GLBuffer {
    pub fn new(context: &GLContext) -> Result<Self, GraphicsError> {
        let mut handle = 0;
        unsafe {
            context.gl.GenBuffers(1, &mut handle);
        }
        
        Ok(Self {
            gl: Arc::clone(&context.gl),
            handle,
            size: 0,
            usage: BufferUsage::default(),
            target: gl::ARRAY_BUFFER,
            stride: 0,
            attributes: Vec::new(),
        })
    }

    pub fn set_vertex_layout(&mut self, layout: &VertexLayout) -> Result<(), GraphicsError> {
        self.stride = layout.stride;
        self.attributes = layout.attributes.clone();
        Ok(())
    }

    pub fn bind_vertex_attributes(&self, vao: gl::types::GLuint) {
        unsafe {
            self.gl.BindVertexArray(vao);
            self.gl.BindBuffer(gl::ARRAY_BUFFER, self.handle);

            for attr in &self.attributes {
                let (size, type_, normalized) = attr.format.to_gl_params();
                
                self.gl.EnableVertexAttribArray(attr.location);
                self.gl.VertexAttribPointer(
                    attr.location,
                    size,
                    type_,
                    normalized,
                    self.stride as gl::types::GLsizei,
                    attr.offset as *const _,
                );
                
                if attr.divisor > 0 {
                    self.gl.VertexAttribDivisor(attr.location, attr.divisor);
                }
            }
        }
    }
}
