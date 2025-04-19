#![no_std]

use allocator::{BaseAllocator, ByteAllocator, PageAllocator, AllocResult, AllocError};
use core::alloc::Layout;
use core::ptr::NonNull;
// use axhal::mem::{phys_to_virt, virt_to_phys};

/// Early memory allocator
/// Use it before formal bytes-allocator and pages-allocator can work!
/// This is a double-end memory range:
/// - Alloc bytes forward
/// - Alloc pages backward
///
/// [ bytes-used | avail-area | pages-used ]
/// |            | -->    <-- |            |
/// start       b_pos        p_pos       end
///
/// For bytes area, 'count' records number of allocations.
/// When it goes down to ZERO, free bytes-used area.
/// For pages area, it will never be freed!
///
pub struct EarlyAllocator<const PAGE_SIZE: usize>{
    /// Start of memory
    start: usize,
    /// End of memory
    end: usize,
    /// Start of avaliable memory
    free_start: usize,
    /// End of avaliable memory
    free_end: usize,
    /// Allocated bytes
    allocated_bytes: usize,
}

impl<const PAGE_SIZE: usize> EarlyAllocator<PAGE_SIZE> {
    /// Create an new allocator
    pub const fn new() -> Self {
        Self {
            start: 0,
            end: 0,
            free_start: 0,
            free_end: 0,
            allocated_bytes: 0,
        }
    }
}

impl<const N: usize> BaseAllocator for EarlyAllocator<N> {
    /// Initialize the allocator with a free memory region.
    fn init(&mut self, start: usize, size: usize) {
        self.start = start;
        self.end = start + size;
        self.free_start = start;
        self.free_end = start + size;
        self.allocated_bytes = 0;
    }

    /// Add a free memory region to the allocator.
    fn add_memory(&mut self, _start: usize, _size: usize) -> AllocResult {
        unimplemented!()
    }
}

impl<const N: usize> ByteAllocator for EarlyAllocator<N> {
    /// Allocate memory with the given size (in bytes) and alignment.
    fn alloc(&mut self, layout: Layout) -> AllocResult<NonNull<u8>> {
        if self.available_bytes() < layout.size() {
            return Err(AllocError::NoMemory);
        }

        let pos = self.free_start;
        self.free_start += layout.size();
        self.allocated_bytes += layout.size();

        Ok(NonNull::<u8>::new(pos as *mut u8).expect("ptr is null!"))
    }

    /// Deallocate memory at the given position, size, and alignment.
    fn dealloc(&mut self, _pos: NonNull<u8>, layout: Layout) {
        // if self.free_start <= usize::from_be_bytes((pos.as_ref() as &[u8]).try_into().expect("slice with incorrect length")) {
        //     return;
        // }

        self.allocated_bytes -= layout.size();
        if self.allocated_bytes == 0 {
            self.free_start = self.start;
        }
    }

    /// Returns total memory size in bytes.
    fn total_bytes(&self) -> usize {
        self.end - self.start
    }

    /// Returns allocated memory size in bytes.
    fn used_bytes(&self) -> usize {
        self.allocated_bytes
    }

    /// Returns available memory size in bytes.
    fn available_bytes(&self) -> usize {
        self.free_end - self.free_start
    }
}

impl<const PAGE_SIZE: usize> PageAllocator for EarlyAllocator<PAGE_SIZE> {
    /// The size of a memory page.
    const PAGE_SIZE: usize = PAGE_SIZE;

    /// Allocate contiguous memory pages with given count and alignment.
    fn alloc_pages(&mut self, num_pages: usize, _align_pow2: usize) -> AllocResult<usize> {
        if self.available_pages() < num_pages {
            return Err(AllocError::NoMemory);
        }

        self.free_end -= num_pages * Self::PAGE_SIZE;

        Ok(self.free_end)
    }

    /// Deallocate contiguous memory pages with given position and count.
    fn dealloc_pages(&mut self, _pos: usize, _num_pages: usize) {
        unimplemented!()
    }

    // Allocate contiguous memory pages with given base address, count and alignment.
    // fn alloc_pages_at(
    //     &mut self,
    //     _base: usize,
    //     _num_pages: usize,
    //     _align_pow2: usize,
    // ) -> AllocResult<usize> {
    //     unimplemented!()
    // }

    /// Returns the total number of memory pages.
    fn total_pages(&self) -> usize {
        (self.end - self.start) / Self::PAGE_SIZE
    }

    /// Returns the number of allocated memory pages.
    fn used_pages(&self) -> usize {
        (self.end - self.free_end) / Self::PAGE_SIZE
    }

    /// Returns the number of available memory pages.
    fn available_pages(&self) -> usize {
        (self.free_end - self.free_start) / Self::PAGE_SIZE
    }
}
