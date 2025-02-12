// File: backends/opengl/command_buffer.rs
use std::sync::Arc;
use gl;
use crate::graphics::{
    command::CommandBuffer,
    core::GraphicsError,
    pipeline::Pipeline,
    buffer::Buffer,
};
use crate::graphics::backends::opengl::{
    OpenGLBackend,
    GLPipeline,
    GLBuffer,
};

pub struct GLCommandBuffer {
    gl: Arc<gl::Gl>,
    // For now, using immediate execution, no command recording needed yet.
    // is_recording: bool, // Could add this later if we want to track recording state
}

impl GLCommandBuffer {
    pub fn new(gl: Arc<gl::Gl>) -> Result<Self, GraphicsError> {
        Ok(Self {
            gl,
            // is_recording: false,
        })
    }
}

impl CommandBuffer for GLCommandBuffer {
    fn begin(&mut self) -> Result<(), GraphicsError> {
        // For immediate execution, begin might not be needed.
        // If we were recording commands, this would start recording.
        // self.is_recording = true;
        Ok(())
    }

    fn end(&mut self) -> Result<(), GraphicsError> {
        // For immediate execution, end might not be needed.
        // If we were recording commands, this would stop recording and prepare for submission.
        // self.is_recording = false;
        Ok(())
    }

    fn bind_pipeline(&mut self, pipeline: &impl Pipeline) -> Result<(), GraphicsError> {
        let gl_pipeline = pipeline.downcast_ref::<GLPipeline>()
            .ok_or(GraphicsError::DowncastFailed)?; // Or a more specific error

        unsafe {
            self.gl.UseProgram(gl_pipeline.program);
        }
        Ok(())
    }

    fn bind_vertex_buffer(&mut self, buffer: &impl Buffer, offset: u64) -> Result<(), GraphicsError> {
        let gl_buffer = buffer.downcast_ref::<GLBuffer>()
            .ok_or(GraphicsError::DowncastFailed)?;

        unsafe {
            self.gl.BindBuffer(gl::ARRAY_BUFFER, gl_buffer.handle);
            // Assuming VAO is already bound and configured for this buffer's layout
            gl_buffer.bind_vertex_attributes(0); // VAO 0 for now, need proper VAO management later
        }
        Ok(())
    }

    fn draw(&mut self, vertex_count: u32, instance_count: u32) -> Result<(), GraphicsError> {
        unsafe {
            if instance_count > 1 {
                self.gl.DrawArraysInstanced(
                    gl::TRIANGLES, // Primitive type - can be parameterized later
                    0,
                    vertex_count as i32,
                    instance_count as i32,
                );
            } else {
                self.gl.DrawArrays(
                    gl::TRIANGLES, // Primitive type - can be parameterized later
                    0,
                    vertex_count as i32,
                );
            }
        }
        Ok(())
    }

    fn draw_indexed(&mut self, index_count: u32, instance_count: u32) -> Result<(), GraphicsError> {
        Err(GraphicsError::NotImplemented) // Implement later
    }
}