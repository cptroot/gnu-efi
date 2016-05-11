use super::types;
use super::super::bind;

use super::types::{FunctionPointer, TableHeader};

#[repr(C)]
pub struct BootServices {
    hdr:					TableHeader,

    //
    // Task priority functions
    //

    RaiseTPL:                   FunctionPointer,
    RestoreTPL:                 FunctionPointer,

    //
    // Memory functions
    //

    AllocatePages:                  FunctionPointer,
    FreePages:                  FunctionPointer,
    GetMemoryMap:                   extern fn(memory_map_size:&mut usize, memory_map:*const def::MemoryDescriptor, map_key:&mut usize, descriptor_size:&mut usize, descriptor_version:&mut u32)->def::Status,
    AllocatePool:                   FunctionPointer,
    FreePool:                   FunctionPointer,

    //
    // Event & timer functions
    //

    CreateEvent:                FunctionPointer,
    SetTimer:                   FunctionPointer,
    WaitForEvent:                   FunctionPointer,
    SignalEvent:                FunctionPointer,
    CloseEvent:                 FunctionPointer,
    CheckEvent:                 FunctionPointer,

    //
    // Protocol handler functions
    //

    InstallProtocolInterface:                   FunctionPointer,
    ReinstallProtocolInterface:                 FunctionPointer,
    UninstallProtocolInterface:                 FunctionPointer,
    HandleProtocol:                 FunctionPointer,
    PCHandleProtocol:                   FunctionPointer,
    RegisterProtocolNotify:                 FunctionPointer,
    LocateHandle:                   FunctionPointer,
    LocateDevicePath:                   FunctionPointer,
    InstallConfigurationTable:                  FunctionPointer,

    //
    // Image functions
    //

    LoadImage:                  FunctionPointer,
    StartImage:                 FunctionPointer,
    Exit:                   FunctionPointer,
    UnloadImage:                FunctionPointer,
    ExitBootServices:                   extern fn(image_handle:*const c_void, map_key:usize)->def::Status,

    //
    // Misc functions
    //

    GetNextMonotonicCount:                  FunctionPointer,
    Stall:                  FunctionPointer,
    SetWatchdogTimer:                   FunctionPointer,

    //
    // DriverSupport Services
    //

    ConnectController:                  FunctionPointer,
    DisconnectController:                   FunctionPointer,

    //
    // Open and Close Protocol Services
    //
    OpenProtocol:                   FunctionPointer,
    CloseProtocol:                  FunctionPointer,
    OpenProtocolInformation:                FunctionPointer,

    //
    // Library Services
    //
    ProtocolsPerHandle:                 FunctionPointer,
    LocateHandleBuffer:                 FunctionPointer,
    LocateProtocol:                 FunctionPointer,
    InstallMultipleProtocolInterfaces:                  FunctionPointer,
    UninstallMultipleProtocolInterfaces:                FunctionPointer,

    //
    // 32-bit CRC Services
    //
    CalculateCrc32:                 FunctionPointer,

    //
    // Misc Services
    //
    CopyMem:                FunctionPointer,
    SetMem:                 FunctionPointer,
    CreateEventEx:                  FunctionPointer,
}

#[repr(C)]
pub struct RuntimeServices {
    hdr:					    TableHeader,

    //
    // Time services
    //
    GetTime:                    FunctionPointer,
    SetTime:                    FunctionPointer,
    GetWakeupTime:              FunctionPointer,
    SetWakeupTime:              FunctionPointer,

    //
    // Virtual memory services
    //
    SetVirtualAddressMap:       FunctionPointer,
    ConvertPointer:             FunctionPointer,

    //
    // Variable serviers
    //
    GetVariable:                FunctionPointer,
    GetNextVariableName:        FunctionPointer,
    SetVariable:                FunctionPointer,

    //
    // Misc
    //
    GetNextHighMonotonicCount:  FunctionPointer,
    ResetSystem:                extern fn(reset_type:types::ResetType, reset_status:def::Status, data_size:usize, reset_data:*const c_void) -> def::Status,
}

use core::mem;
use def;
use libc::c_void;

impl BootServices {
    pub fn get_memory_map(&self) -> (&'static [def::MemoryDescriptor], usize) {
        let mut memory_map_size = mem::size_of::<def::MemoryDescriptor>() * 4;
        unsafe {
            let mut memory_map: &'static mut [def::MemoryDescriptor] = mem::uninitialized();
            let mut map_key = mem::uninitialized();
            let mut descriptor_size = mem::uninitialized();
            let mut descriptor_version = mem::uninitialized();

            let status = bind::safe_efi_call5(
                self.GetMemoryMap,
                    &mut memory_map_size,
                    memory_map.as_mut_ptr(),
                    &mut map_key,
                    &mut descriptor_size,
                    &mut descriptor_version);

            if status != def::Status::Success {
                panic!("Unable to get memory map: {:?}", status);
            }

            (memory_map, map_key)
        }
    }

    pub fn exit_boot_services(&self, image_handle: def::Handle, map_key: usize) {
        let status = bind::safe_efi_call2(
            self.ExitBootServices,
            image_handle.handle,
            map_key);
        if status != def::Status::Success {
            panic!("Unable to exit boot services: {:?}", status);
        }
    }
}

impl RuntimeServices {
    pub fn reset_system(&self, reset_type:types::ResetType, reset_status:def::Status, data_size:usize, reset_data:*const c_void) -> () {
        bind::safe_efi_call4(
            self.ResetSystem,
            reset_type,
            reset_status,
            data_size,
            reset_data);
    }
}
