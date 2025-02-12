
// src/graphics/backends/opengl/pipeline.rs
pub struct GLPipeline {
    gl: Arc<gl::Gl>,
    program: gl::types::GLuint,
    vertex_layout: VertexLayout,
    state: PipelineState,
    uniforms: HashMap<String, UniformLocation>,
    uniform_blocks: HashMap<String, UniformBlockInfo>,
    fast_uniforms: FastUniformCache,
}

impl GLPipeline {
    pub fn new(context: &GLContext) -> Result<Self, GraphicsError> {
        let program = unsafe { context.gl.CreateProgram() };
        
        Ok(Self {
            gl: Arc::clone(&context.gl),
            program,
            vertex_layout: VertexLayout::default(),
            state: PipelineState::default(),
            uniforms: HashMap::new(),
            uniform_blocks: HashMap::new(),
            fast_uniforms: FastUniformCache::default(),
        })
    }

    pub fn reflect_uniforms(&mut self) -> Result<(), GraphicsError> {
        unsafe {
            let mut count = 0;
            self.gl.GetProgramiv(self.program, gl::ACTIVE_UNIFORMS, &mut count);

            for i in 0..count {
                let mut name = [0u8; 256];
                let mut length = 0;
                let mut size = 0;
                let mut type_ = 0;

                self.gl.GetActiveUniform(
                    self.program,
                    i as gl::types::GLuint,
                    256,
                    &mut length,
                    &mut size,
                    &mut type_,
                    name.as_mut_ptr() as *mut gl::types::GLchar,
                );

                let name = std::str::from_utf8(&name[..length as usize])
                    .map_err(|_| GraphicsError::UniformReflectionFailed)?;
                
                let location = self.gl.GetUniformLocation(self.program, name.as_ptr() as *const _);
                if location >= 0 {
                    self.uniforms.insert(name.to_string(), UniformLocation(location));
                }
            }
        }
        Ok(())
    }

    pub fn reflect_uniform_blocks(&mut self) -> Result<(), GraphicsError> {
        unsafe {
            let mut count = 0;
            self.gl.GetProgramiv(self.program, gl::ACTIVE_UNIFORM_BLOCKS, &mut count);

            for i in 0..count {
                let mut name = [0u8; 256];
                let mut length = 0;
                let mut size = 0;

                self.gl.GetActiveUniformBlockName(
                    self.program,
                    i as gl::types::GLuint,
                    256,
                    &mut length,
                    name.as_mut_ptr() as *mut gl::types::GLchar,
                );

                self.gl.GetActiveUniformBlockiv(
                    self.program,
                    i as gl::types::GLuint,
                    gl::UNIFORM_BLOCK_DATA_SIZE,
                    &mut size,
                );

                let name = std::str::from_utf8(&name[..length as usize])
                    .map_err(|_| GraphicsError::UniformReflectionFailed)?;

                self.uniform_blocks.insert(name.to_string(), UniformBlockInfo {
                    index: i as gl::types::GLuint,
                    size: size as usize,
                    binding: i as u32, // Default binding point
                });
            }
        }
        Ok(())
    }

    // Push-constant-like functionality using uniform buffers
    pub fn push_constants<T: Copy>(&mut self, data: &T, offset: usize) -> Result<(), GraphicsError> {
        let size = std::mem::size_of::<T>();
        if self.fast_uniforms.buffer.len() < offset + size {
            self.fast_uniforms.buffer.resize(offset + size, 0);
        }

        unsafe {
            std::ptr::copy_nonoverlapping(
                data as *const T as *const u8,
                self.fast_uniforms.buffer.as_mut_ptr().add(offset),
                size,
            );
        }

        // Update uniforms immediately for push-constant-like behavior
        self.update_fast_uniforms()?;
        Ok(())
    }

    fn update_fast_uniforms(&self) -> Result<(), GraphicsError> {
        unsafe {
            // Use a dedicated uniform buffer for fast updates
            // This mimics push constant behavior while using UBOs under the hood
            self.gl.BufferSubData(
                gl::UNIFORM_BUFFER,
                0,
                self.fast_uniforms.buffer.len() as gl::types::GLsizeiptr,
                self.fast_uniforms.buffer.as_ptr() as *const _,
            );
        }
        Ok(())
    }
}
