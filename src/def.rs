

use libc::c_void;
pub use err::Status;

pub struct Handle {
    pub handle:*const c_void,
}

#[repr(C)]
#[derive(Debug)]
pub struct MemoryDescriptor {
    pub region_type:        u32,
    pub physical_start:     PhysicalAddress,
    pub virtual_start:      VirtualAddress,
    pub number_of_pages:    u64,
    pub attribute:          u64,
}

#[repr(C)]
#[derive(Debug)]
pub struct VirtualAddress {
    address:u64,
}

#[repr(C)]
#[derive(Debug)]
pub struct PhysicalAddress {
    address:u64,
}

