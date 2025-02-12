// src/graphics/backends/opengl/types.rs
#[derive(Debug, Clone)]
pub struct VertexAttribute {
    location: u32,
    format: AttributeFormat,
    offset: usize,
    divisor: u32,
}

#[derive(Debug, Clone)]
pub enum AttributeFormat {
    Float1,
    Float2,
    Float3,
    Float4,
    Int1,
    Int2,
    Int3,
    Int4,
    UInt1,
    UInt2,
    UInt3,
    UInt4,
}
