// src/memory/frame.rs
pub struct FrameAllocator {
    // Double/Triple buffered memory for frame data
    buffers: Vec<Vec<u8>>,
    current_frame: usize,
    frame_count: usize,
}

impl FrameAllocator {
    pub fn new(frame_count: usize, buffer_size: usize) -> Self {
        Self {
            buffers: vec![vec![0; buffer_size]; frame_count],
            current_frame: 0,
            frame_count,
        }
    }
    
    pub fn next_frame(&mut self) {
        self.current_frame = (self.current_frame + 1) % self.frame_count;
    }
}