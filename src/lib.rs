#![no_std]

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

extern crate libc;

mod err;

pub mod def;

pub fn strlen(ptr:*const u8) -> usize {
    let mut iter = ptr;
    unsafe {
        while *iter != 0 {
            iter = iter.offset(1);
        }
    }
    return (iter as usize) - (ptr as usize);
}

pub mod bind;
pub mod efilib;
pub mod api;


