
extern crate libc;
use libc::{c_void, c_char, malloc, free};
use std::ffi::CStr;

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

fn dummy_ffifuncproxy_placeholder(v: &ffiparam) {}
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
    unsafe {
    let xs = cstrfrom_u8ptr(ptr, len);
    
    return xs.to_string()
    }
}
pub fn cstrfrom_usizeptr(ptrx :usize, len: usize) -> String {
    let ptr = ptrx as *const u8;
    unsafe {
    let xs = cstrfrom_u8ptr(ptr, len);
    
    return xs.to_string()
    }
}
pub fn refstr2cstru8(v : &str) -> *const u8 {
    v.as_ptr()
}
pub fn refstr2cstrusize(v : &str) -> usize {
    v.as_ptr() as usize
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
