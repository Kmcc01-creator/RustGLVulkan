// src/graphics/backends/opengl/shader.rs
pub struct GLShader {
    gl: Arc<gl::Gl>,
    handle: gl::types::GLuint,
    stage: ShaderStage,
}

impl GLShader {
    pub fn new(context: &GLContext, stage: ShaderStage) -> Result<Self, GraphicsError> {
        let handle = unsafe {
            context.gl.CreateShader(stage.to_gl_enum())
        };
        
        Ok(Self {
            gl: Arc::clone(&context.gl),
            handle,
            stage,
        })
    }
    
    pub fn compile(&self, source: &str) -> Result<(), GraphicsError> {
        unsafe {
            let source = std::ffi::CString::new(source)
                .map_err(|_| GraphicsError::ShaderCompilationFailed)?;
            
            self.gl.ShaderSource(
                self.handle,
                1,
                &source.as_ptr(),
                std::ptr::null(),
            );
            
            self.gl.CompileShader(self.handle);
            
            // Check compilation status
            let mut success = 0;
            self.gl.GetShaderiv(self.handle, gl::COMPILE_STATUS, &mut success);
            
            if success == 0 {
                let mut log_length = 0;
                self.gl.GetShaderiv(self.handle, gl::INFO_LOG_LENGTH, &mut log_length);
                
                let mut log = Vec::with_capacity(log_length as usize);
                self.gl.GetShaderInfoLog(
                    self.handle,
                    log_length,
                    std::ptr::null_mut(),
                    log.as_mut_ptr() as *mut gl::types::GLchar,
                );
                
                log.set_len(log_length as usize);
                let error_message = String::from_utf8_lossy(&log);
                return Err(GraphicsError::ShaderCompilationFailed);
            }
        }
        
        Ok(())
    }
}
