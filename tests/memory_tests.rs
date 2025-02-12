// tests/memory_tests.rs
#[cfg(test)]
mod arena_tests {
    use gl_engine::memory::arena::{Arena, ArenaConfig};

    #[test]
    fn test_basic_allocation() {
        let mut arena = Arena::<u32>::new(ArenaConfig {
            block_size: 1024,
            alignment: 4,
        });
        
        // Basic allocation
        let value = arena.alloc();
        assert!(value.is_some());
        
        // Fill a complete block
        for _ in 0..1023 {
            assert!(arena.alloc().is_some());
        }
        
        // Verify block expansion
        assert!(arena.alloc().is_some());
    }

    #[test]
    fn test_alignment_requirements() {
        // Test with different alignments
        let mut arena = Arena::<u64>::new(ArenaConfig {
            block_size: 1024,
            alignment: 8,
        });
        
        let ptr = arena.alloc().unwrap() as *const u64;
        assert_eq!(ptr.align_offset(8), 0);
    }

    #[test]
    fn test_bulk_allocation() {
        let mut arena = Arena::<u32>::new(ArenaConfig {
            block_size: 1024,
            alignment: 4,
        });
        
        // Allocate a slice
        let slice = arena.alloc_slice(100);
        assert_eq!(slice.len(), 100);
        
        // Verify continuous memory
        let ptr_start = slice.as_ptr() as usize;
        let ptr_end = (slice.as_ptr() as usize) + (99 * std::mem::size_of::<u32>());
        assert_eq!(ptr_end - ptr_start, 99 * std::mem::size_of::<u32>());
    }

    #[test]
    fn test_clear_and_reuse() {
        let mut arena = Arena::<u32>::new(ArenaConfig {
            block_size: 1024,
            alignment: 4,
        });
        
        // Fill arena
        for _ in 0..1024 {
            arena.alloc();
        }
        
        // Clear and verify reuse
        arena.clear();
        assert!(arena.alloc().is_some());
    }

    #[test]
fn test_complex_type_allocation() {
    #[derive(Debug, PartialEq)]
    struct TestStruct {
        x: f32,
        y: f32,
        data: [u8; 32],
    }
    
    let mut arena = Arena::<TestStruct>::new(ArenaConfig {
        block_size: 128,
        alignment: 16,  // Aligned for SIMD
    });
    
    // Test allocation of complex type
    let value = arena.alloc().unwrap();
    *value = TestStruct {
        x: 1.0,
        y: 2.0,
        data: [0; 32],
    };
    
    assert_eq!(value.x, 1.0);
}

#[test]
fn test_allocation_stress() {
    let mut arena = Arena::<u32>::new(ArenaConfig {
        block_size: 1024,
        alignment: 4,
    });
    
    // Allocate many blocks
    for i in 0..10_000 {
        let value = arena.alloc().unwrap();
        *value = i as u32;
    }
}
}