use core::alloc::GlobalAlloc;

pub struct NitroAllocator;

const OS_CURRENT_HEAP_HANDLE: i32 = -1;

unsafe impl GlobalAlloc for NitroAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        nitro_sys::OS_AllocFromArenaLo(
            nitro_sys::OSArenaId::OS_ARENA_MAIN,
            layout.size() as _,
            layout.align() as _,
        ) as _
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
        nitro_sys::OS_FreeToHeap(
            nitro_sys::OSArenaId::OS_ARENA_MAIN,
            OS_CURRENT_HEAP_HANDLE,
            ptr as _,
        );
    }
}
