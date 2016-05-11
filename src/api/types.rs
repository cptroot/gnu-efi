
use ::def;

pub use super::services::{RuntimeServices, BootServices};


#[repr(C)]
pub struct TableHeader {
    signature: u64,
    revision: u32,
    header_size: u32,
    crc32: u32,
    reserved: u32,
}

#[repr(usize)]
#[derive(Clone, Copy)]
pub enum ResetType {
    ResetCold,
    ResetWarm,
    ResetShutdown,
    ResetPlatformSpecific,
}

impl ::bind::EfiParameter for ResetType {
    fn as_usize(&self) -> usize {
        *self as usize
    }
}

pub type FunctionPointer = def::Handle;


type SIMPLE_INPUT_INTERFACE = def::Handle;
type SIMPLE_TEXT_OUTPUT_INTERFACE = def::Handle;
type EFI_RUNTIME_SERVICES = def::Handle;
type EFI_CONFIGURATION_TABLE = def::Handle;

#[repr(C)]
pub struct SystemTable {
    hdr:                            TableHeader,

    pub firmware_vendor:            *const u16,
    pub firmware_revision:          u32,

    ConsoleInHandle:                def::Handle,
    ConIn: &'static SIMPLE_INPUT_INTERFACE,

    ConsoleOutHandle:               def::Handle,
    ConOut:&'static SIMPLE_TEXT_OUTPUT_INTERFACE,

    StandardErrorHandle:def::Handle,
    StdErr:&'static SIMPLE_TEXT_OUTPUT_INTERFACE,

    pub runtime_services:           &'static RuntimeServices,
    pub boot_services:              &'static BootServices,

    pub number_of_table_entries:    usize,
    configuration_table:            &'static EFI_CONFIGURATION_TABLE,

}

