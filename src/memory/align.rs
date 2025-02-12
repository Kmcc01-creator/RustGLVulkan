// src/memory/align.rs
pub fn align_up(value: usize, alignment: usize) -> usize {
    (value + alignment - 1) & !(alignment - 1)
}

pub struct AlignedBuffer {
    data: Vec<u8>,
    alignment: usize,
}