// File: backends/opengl/texture.rs
use std::sync::Arc;
use gl;
use crate::graphics::{
    core::{GraphicsError, TextureDesc, TextureFormat, TextureType},
};
use crate::graphics::backends::opengl::GLContext;

pub struct GLTexture {
    gl: Arc<gl::Gl>,
    pub handle: gl::types::GLuint,
    pub width: u32,
    pub height: u32,
    pub depth: u32, // For 3D textures if needed later
    pub format: TextureFormat,
    pub type_: TextureType,
    // Add more texture parameters if needed (e.g., mip levels, samples)
}

impl GLTexture {
    pub fn new(context: &GLContext, desc: &TextureDesc) -> Result<Self, GraphicsError> {
        let mut handle: gl::types::GLuint = 0;
        unsafe {
            context.gl.GenTextures(1, &mut handle);
            context.gl.BindTexture(gl::TEXTURE_2D, handle); // For now, assuming TEXTURE_2D

            // Set default texture parameters - can be extended based on TextureDesc later
            context.gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            context.gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            context.gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            context.gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        }

        Ok(Self {
            gl: Arc::clone(&context.gl),
            handle,
            width: desc.width,
            height: desc.height,
            depth: desc.depth, // Assuming TextureDesc has width, height, depth
            format: desc.format,
            type_: desc.type_,
        })
    }

    pub fn allocate(&mut self, _desc: &TextureDesc) -> Result<(), GraphicsError> {
        // For initial implementation, allocation is done in new()
        // If separate allocation needed later (memory management etc.), implement here
        Ok(())
    }

    pub fn upload_data(&mut self, data: &[u8]) -> Result<(), GraphicsError> {
        unsafe {
            self.gl.BindTexture(gl::TEXTURE_2D, self.handle);

            // Determine OpenGL format and type from TextureFormat and TextureType
            let (internal_format, format, type_) = match (self.format, &self.type_) {
                (TextureFormat::RGBA8Unorm, TextureType::U8) => (gl::RGBA8 as i32, gl::RGBA, gl::UNSIGNED_BYTE),
                (TextureFormat::RGBA8Srgb, TextureType::U8) => (gl::SRGB8_ALPHA8 as i32, gl::RGBA, gl::UNSIGNED_BYTE),
                // Add more formats and types as needed, handle errors for unsupported formats
                _ => return Err(GraphicsError::NotImplemented), // For now, handle only RGBA8Unorm/Srgb U8
            };

            self.gl.TexImage2D(
                gl::TEXTURE_2D,
                0, // mip level
                internal_format,
                self.width as i32,
                self.height as i32,
                0, // border - must be 0
                format,
                type_,
                data.as_ptr() as *const _,
            );
        }
        Ok(())
    }

    // Example for mipmap generation (implement later if needed)
    // pub fn generate_mipmaps(&mut self) -> Result<(), GraphicsError> {
    //     unsafe {
    //         self.gl.BindTexture(gl::TEXTURE_2D, self.handle);
    //         self.gl.GenerateMipmap(gl::TEXTURE_2D);
    //     }
    //     Ok(())
    // }
}

impl Drop for GLTexture {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteTextures(1, &self.handle);
        }
    }
}