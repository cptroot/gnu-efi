use core::mem;
use core::slice;

#[link(name = "gnuefi")]
extern {
    fn Print(fmt: *const i16, ...) -> usize;
    fn LibMemoryMap(no_entries: *const usize,
                    map_key: *const usize,
                    descriptor_size: *const usize,
                    descriptor_version: *const u32)
        -> *const ::def::MemoryDescriptor;
}

pub fn print(fmt:&'static str) {
    assert!(fmt.len() < 64);
    let mut wide_fmt = [0i16; 64];
    for (i, c) in fmt.bytes().enumerate() {
        wide_fmt[i] = c as i16;
    }
    unsafe {
        Print(wide_fmt.as_ptr());
    }
}

pub fn lib_memory_map() -> (&'static [::def::MemoryDescriptor], usize) {
    unsafe {
        let mut no_entries = mem::uninitialized();
        let mut map_key = mem::uninitialized();
        let mut descriptor_size = mem::uninitialized();
        let mut descriptor_version = mem::uninitialized();
        let memory_map_ptr = LibMemoryMap(
            &mut no_entries,
            &mut map_key,
            &mut descriptor_size,
            &mut descriptor_version);
        (slice::from_raw_parts(memory_map_ptr, no_entries), map_key)
    }
}
