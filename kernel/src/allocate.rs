use uefi::table::boot::MemoryMapIter;

use kernel_lib::allocator::bitmap_memory_allocator::BitmapFrameAllocator;
use kernel_lib::allocator::memory_map::frame_iter::FrameIter;
use kernel_lib::error::AllocateReason::InitializeGlobalAllocator;
use kernel_lib::error::{KernelError, KernelResult};

#[global_allocator]
static mut HEAP: BitmapFrameAllocator = BitmapFrameAllocator::uninit();

pub fn init_alloc(memory_map: MemoryMapIter<'static>) -> KernelResult {
    unsafe {
        let memory_map_range = FrameIter::new(memory_map)
            .ok_or(KernelError::FailedAllocate(InitializeGlobalAllocator))?;


        const FRAMES: usize = 512 * 32;
        HEAP.init_heap(FRAMES, memory_map_range)
    }
}
