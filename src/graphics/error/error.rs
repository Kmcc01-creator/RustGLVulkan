// src/graphics/error/error.rs

use std::fmt;

#[derive(Debug)]
pub enum GraphicsError {
    VersionDetectionFailed,
    FeatureDetectionFailed,
    ShaderCompilationFailed,
    PipelineCreationFailed,
    BufferCreationFailed,
    TextureCreationFailed,
    CommandRecordingError,
    UniformBindingError,
    UniformReflectionFailed,
    DowncastFailed,
    NotImplemented,
    InvalidOperation, // Generic invalid operation
    OutOfMemory,
    UnknownError,
    BackendError(String), // Generic backend error, can hold backend-specific messages
}

impl fmt::Display for GraphicsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GraphicsError::VersionDetectionFailed => write!(f, "Failed to detect graphics API version."),
            GraphicsError::FeatureDetectionFailed => write!(f, "Failed to detect graphics API features."),
            GraphicsError::ShaderCompilationFailed => write!(f, "Shader compilation failed."),
            GraphicsError::PipelineCreationFailed => write!(f, "Pipeline creation failed."),
            GraphicsError::BufferCreationFailed => write!(f, "Buffer creation failed."),
            GraphicsError::TextureCreationFailed => write!(f, "Texture creation failed."),
            GraphicsError::CommandRecordingError => write!(f, "Error during command recording."),
            GraphicsError::UniformBindingError => write!(f, "Error binding uniform."),
            GraphicsError::UniformReflectionFailed => write!(f, "Uniform reflection failed."),
            GraphicsError::DowncastFailed => write!(f, "Failed to downcast type."),
            GraphicsError::NotImplemented => write!(f, "Functionality not yet implemented."),
            GraphicsError::InvalidOperation => write!(f, "Invalid graphics operation."),
            GraphicsError::OutOfMemory => write!(f, "Out of graphics memory."),
            GraphicsError::UnknownError => write!(f, "An unknown graphics error occurred."),
            GraphicsError::BackendError(msg) => write!(f, "Backend error: {}", msg),
        }
    }
}

impl std::error::Error for GraphicsError {}