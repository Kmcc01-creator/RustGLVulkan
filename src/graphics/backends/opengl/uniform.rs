// src/graphics/backends/opengl/uniform.rs
#[derive(Debug)]
pub struct UniformLocation(gl::types::GLint);

#[derive(Debug)]
pub struct UniformBlockInfo {
    index: gl::types::GLuint,
    size: usize,
    binding: u32,
}

#[derive(Default)]
pub struct FastUniformCache {
    vec4_cache: HashMap<u32, [f32; 4]>,
    mat4_cache: HashMap<u32, [f32; 16]>,
    buffer: Vec<u8>, // For staging push-constant-like data
}