// src/memory/gpu.rs
pub struct GPUBuffer {
    handle: gl::types::GLuint,
    size: usize,
    usage: gl::types::GLenum,
}

impl GPUBuffer {
    pub fn new(size: usize, usage: gl::types::GLenum) -> Result<Self, String> {
        let mut handle = 0;
        unsafe {
            gl::GenBuffers(1, &mut handle);
            gl::BindBuffer(gl::ARRAY_BUFFER, handle);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                size as gl::types::GLsizeiptr,
                std::ptr::null(),
                usage,
            );
        }
        // ... error handling and buffer creation
        Ok(Self { handle, size, usage })
    }
}