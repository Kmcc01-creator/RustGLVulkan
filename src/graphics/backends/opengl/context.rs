// src/graphics/backends/opengl/context.rs
use gl;
use std::sync::Arc;

pub struct GLContext {
    gl: Arc<gl::Gl>,
    version: GLVersion,
    features: GLFeatureSet,
    state_cache: GLStateCache,
}

impl GLContext {
    pub fn new(gl: gl::Gl) -> Result<Self, GraphicsError> {
        let gl = Arc::new(gl);
        let version = Self::detect_version(&gl)?;
        let features = Self::detect_features(&gl)?;
        
        Ok(Self {
            gl,
            version,
            features,
            state_cache: GLStateCache::new(),
        })
    }
    
    fn detect_version(gl: &gl::Gl) -> Result<GLVersion, GraphicsError> {
        unsafe {
            let version = std::ffi::CStr::from_ptr(gl.GetString(gl::VERSION) as *const _)
                .to_str()
                .map_err(|_| GraphicsError::VersionDetectionFailed)?;
            GLVersion::parse(version)
        }
    }
    
    fn detect_features(gl: &gl::Gl) -> Result<GLFeatureSet, GraphicsError> {
        let mut features = GLFeatureSet::default();
        unsafe {
            let extensions = gl.GetString(gl::EXTENSIONS);
            // Parse and populate features based on extensions
        }
        Ok(features)
    }
}
