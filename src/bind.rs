
use core::mem;
use ::def;

pub trait EfiParameter {
    fn as_usize(&self) -> usize;
}

impl EfiParameter for usize {
    fn as_usize(&self) -> usize {
        *self
    }
}

impl<T> EfiParameter for *const T {
    fn as_usize(&self) -> usize {
        unsafe { mem::transmute(*self) }
    }
}

impl<'a, T> EfiParameter for &'a mut T {
    fn as_usize(&self) -> usize {
        unsafe {
            let ptr: *const T = *self;
            mem::transmute(ptr)
        }
    }
}

impl EfiParameter for def::Status {
    fn as_usize(&self) -> usize {
        *self as usize
    }
}

#[link(name = "gnuefi")]
extern "C" {
    fn efi_call2(func:extern fn(a:usize, b:usize), a:usize, b:usize) -> usize;
    fn efi_call3(func:extern fn(a:usize, b:usize, c:usize), a:usize, b:usize, c:usize) -> usize;
    fn efi_call4(func:extern fn(a:usize, b:usize, c:usize, d:usize), a:usize, b:usize, c:usize, d:usize) -> usize;
    fn efi_call5(func:extern fn(a:usize, b:usize, c:usize, d:usize, e:usize), a:usize, b:usize, c:usize, d:usize, e:usize) -> usize;
}

pub fn safe_efi_call2<U, V>(f:extern fn(U, V) -> def::Status,
                        u:U, v:V)
    -> def::Status
        where U: EfiParameter,
              V: EfiParameter,
{
    unsafe {
        mem::transmute(efi_call2(
            mem::transmute(f),
            u.as_usize(),
            v.as_usize()))
    }
}

pub fn safe_efi_call3<U, V, W>(f:extern fn(U, V, W) -> def::Status,
                              u:U, v:V, w:W)
    -> def::Status
        where U: EfiParameter,
              V: EfiParameter,
              W: EfiParameter,
{
    unsafe {
        mem::transmute(efi_call3(
            mem::transmute(f),
            u.as_usize(),
            v.as_usize(),
            w.as_usize()))
    }
}

pub fn safe_efi_call4<U, V, W, X>(f:extern fn(U, V, W, X) -> def::Status,
                              u:U, v:V, w:W, x:X)
    -> def::Status
        where U: EfiParameter,
              V: EfiParameter,
              W: EfiParameter,
              X: EfiParameter,
{
    unsafe {
        mem::transmute(efi_call4(
            mem::transmute(f),
            u.as_usize(),
            v.as_usize(),
            w.as_usize(),
            x.as_usize()))
    }
}

pub fn safe_efi_call5<U, V, W, X, Y>(f:extern fn(U, V, W, X, Y) -> def::Status,
                              u:U, v:V, w:W, x:X, y:Y)
    -> def::Status
        where U: EfiParameter,
              V: EfiParameter,
              W: EfiParameter,
              X: EfiParameter,
              Y: EfiParameter,
{
    unsafe {
        mem::transmute(efi_call5(
            mem::transmute(f),
            u.as_usize(),
            v.as_usize(),
            w.as_usize(),
            x.as_usize(),
            y.as_usize()))
    }
}

