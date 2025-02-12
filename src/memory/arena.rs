// src/memory/arena.rs
use std::mem::MaybeUninit;
use std::alloc::{Layout, alloc, dealloc};

#[derive(Debug)]
pub struct ArenaConfig {
    pub block_size: usize,
    pub alignment: usize,
}

pub struct Arena<T> {
    blocks: Vec<Box<[MaybeUninit<T>]>>,
    current_block: usize,
    next_allocation: usize,
    config: ArenaConfig,
}

impl<T> Arena<T> {
    pub fn new(config: ArenaConfig) -> Self {
        assert!(config.block_size > 0);
        assert!(config.alignment.is_power_of_two());
        
        Self {
            blocks: Vec::new(),
            current_block: 0,
            next_allocation: 0,
            config,
        }
    }


// SIMD-optimized operations
impl<T> Arena<T> {
    #[cfg(target_feature = "avx2")]
    pub unsafe fn bulk_init_zeros(&mut self, count: usize) -> &mut [T] 
    where T: Copy + Default {
        let slice = self.alloc_slice(count);
        let ptr = slice.as_mut_ptr() as *mut __m256i;
        
        // Using AVX2 for 32-byte zero initialization
        let zero = _mm256_setzero_si256();
        
        for i in 0..(count * std::mem::size_of::<T>() / 32) {
            _mm256_store_si256(ptr.add(i), zero);
        }
        
        // Handle remaining bytes
        let remainder = (count * std::mem::size_of::<T>()) % 32;
        if remainder > 0 {
            let remainder_ptr = (ptr as *mut u8).add(count - remainder);
            std::ptr::write_bytes(remainder_ptr, 0, remainder);
        }
        
        slice
    }

    #[cfg(target_feature = "avx2")]
    pub unsafe fn bulk_copy<'a>(&'a mut self, src: &[T], count: usize) -> &'a mut [T]
    where T: Copy {
        let dst = self.alloc_slice(count);
        let src_ptr = src.as_ptr() as *const __m256i;
        let dst_ptr = dst.as_mut_ptr() as *mut __m256i;
        
        // AVX2 copy
        for i in 0..(count * std::mem::size_of::<T>() / 32) {
            let data = _mm256_load_si256(src_ptr.add(i));
            _mm256_store_si256(dst_ptr.add(i), data);
        }
        
        // Handle remainder
        let remainder = (count * std::mem::size_of::<T>()) % 32;
        if remainder > 0 {
            let remainder_src = (src_ptr as *const u8).add(count - remainder);
            let remainder_dst = (dst_ptr as *mut u8).add(count - remainder);
            std::ptr::copy_nonoverlapping(remainder_src, remainder_dst, remainder);
        }
        
        dst
    }
}
    
    pub fn alloc(&mut self) -> Option<&mut T> {
        if self.next_allocation >= self.config.block_size {
            self.add_block()?;
        }
        
        let block = self.blocks.get_mut(self.current_block)?;
        let value = &mut block[self.next_allocation];
        self.next_allocation += 1;
        
        // Safety: we know this location is uninitialized
        Some(unsafe { 
            std::mem::transmute(value)
        })
    }
    
    pub fn alloc_slice(&mut self, size: usize) -> &mut [T] {
        // Ensure we have enough space
        if self.next_allocation + size > self.config.block_size {
            self.add_block().expect("Failed to allocate new block");
        }
        
        let start = self.next_allocation;
        self.next_allocation += size;
        
        let block = &mut self.blocks[self.current_block];
        // Safety: we know this range is uninitialized
        unsafe {
            std::slice::from_raw_parts_mut(
                &mut block[start] as *mut MaybeUninit<T> as *mut T,
                size
            )
        }
    }
    
    fn add_block(&mut self) -> Option<()> {
        let layout = Layout::array::<T>(self.config.block_size)
            .ok()?
            .align_to(self.config.alignment)
            .ok()?;
            
        let block = unsafe {
            let ptr = alloc(layout) as *mut MaybeUninit<T>;
            Box::from_raw(std::slice::from_raw_parts_mut(
                ptr,
                self.config.block_size
            ))
        };
        
        self.blocks.push(block);
        self.current_block = self.blocks.len() - 1;
        self.next_allocation = 0;
        
        Some(())
    }
    
    pub fn clear(&mut self) {
        // Reset allocation counters
        self.current_block = 0;
        self.next_allocation = 0;
        
        // Note: Memory is not deallocated, just marked as reusable
    }
}

impl<T> Drop for Arena<T> {
    fn drop(&mut self) {
        // Clean up allocated blocks
        for block in self.blocks.drain(..) {
            let layout = Layout::array::<T>(self.config.block_size)
                .unwrap()
                .align_to(self.config.alignment)
                .unwrap();
                
            unsafe {
                dealloc(
                    Box::into_raw(block) as *mut u8,
                    layout
                );
            }
        }
    }
}

// Tracking macro for debug builds
#[macro_export]
macro_rules! track_alloc {
    ($arena:expr, $size:expr) => {
        {
            #[cfg(debug_assertions)]
            $arena.metrics.record_trace(concat!(file!(), ":", line!()), $size);
            $arena.alloc_slice($size)
        }
    };
}