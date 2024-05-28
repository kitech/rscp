
extern crate libc;
use libc::{c_void, c_char}; //, malloc, free};
use std::ffi::CStr;
use std::ffi::CString;

// the trait `std::default::Default` is implemented for `u8`
#[derive(Default)]
#[repr(C)]
pub struct ffiparam {
    // ptr: *const u8,
    ptr: usize,
    len: usize,

    // resp: *const u8,
    resp: usize,
    len2: usize,
}

fn dummy_ffifuncproxy_placeholder(_v: &ffiparam) {}
static mut ffifuncproxy_rs2go : fn(v:&ffiparam) = dummy_ffifuncproxy_placeholder;

/////////////
pub fn cstrfrom_u8ptr(ptr :*const u8, len: usize) -> &'static str {
    unsafe {
    let vcc  = core::slice::from_raw_parts(ptr, len+1);
    // println!("xxxx {}", vcc.len());
    let x = CStr::from_bytes_with_nul_unchecked(vcc);
    let xs = x.to_str().to_owned().unwrap();
    
    return xs
    }
}
pub fn cstrfrom_u8ptr2(ptr :*const u8, len: usize) -> String {
    // unsafe {
    let xs = cstrfrom_u8ptr(ptr, len);
    
    return xs.to_string()
    // }
}
pub fn cstrfrom_usizeptr(ptrx :usize, len: usize) -> String {
    let ptr = ptrx as *const u8;
    // unsafe {
    let xs = cstrfrom_u8ptr(ptr, len);
    
    return xs.to_string()
    // }
}
// todo
// pub fn CStringfrom_u8ptr(ptrx :*const u8, len: usize) -> CString {
    // let vcc  = core::slice::from_raw_parts(ptrx, len+1);
    // vcc.into()
    // return ()
// }
pub fn refstr2_cstru8(v : &str) -> *const u8 {
    v.as_ptr()
}
pub fn refstr2_cstrusize(v : &str) -> usize {
    v.as_ptr() as usize
}

// c memory functions
pub fn cfree_usize(ptrx : usize) {
    let ptr = ptrx as *mut c_void;
    unsafe { libc::free(ptr); }
}
pub fn cfree_cvoid(ptr : *mut c_void) {
    unsafe { libc::free(ptr); }
}
pub fn cfree_u8ptr(ptrx : *const u8) {
    let ptr = ptrx as *mut c_void;
    unsafe { libc::free(ptr); }
}
pub fn cfree_cchar(ptrx : *const c_char) {
    let ptr = ptrx as *mut c_void;
    unsafe { libc::free(ptr); }
}
// todo
pub fn cfree<T>(ptrx : T) {
    // let ptr = ptrx as *mut c_void;
    // unsafe { libc::free(ptr); }
}

/////////// template
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
