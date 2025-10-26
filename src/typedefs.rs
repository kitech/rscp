extern crate libc;
use libc::{c_char, c_void}; //, malloc, free};

// use std::ffi::CStr;
// use std::ffi::CString;
// // use std::any;
// use once_cell::sync::Lazy;
// use rustc_demangle::demangle;
// use std::any::Any;
// use std::collections::HashMap;
// use std::fmt;

// some type alias
#[allow(non_camel_case_types)]
pub type voidptr = *mut c_void;
#[allow(non_camel_case_types)]
pub type voidptrc = *const c_void;
#[allow(non_camel_case_types)]
pub type charptr = *mut i8;
#[allow(non_camel_case_types)]
pub type charptrc = *const i8;
#[allow(non_camel_case_types)]
pub type byteptr = *mut u8;
#[allow(non_camel_case_types)]
pub type byteptrc = *const u8;

// \see std::ptr::null_mut()
pub const nilvoidptr: *mut c_void = 0 as *mut c_void;
pub const nilcharptr: *mut c_char = 0 as *mut c_char;
//pub const NILWCHARPTRM : *mut libc::wchar_t = 0 as *mut libc::wchar_t;
pub const nili8ptr: *mut i8 = 0 as *mut i8;
pub const nilu8ptr: *mut u8 = 0 as *mut u8;
pub const nilvoidptrc: *const c_void = 0 as *const c_void;
pub const nilcharptrc: *const c_char = 0 as *const c_char;
pub const nili8ptrc: *const i8 = 0 as *const i8;
pub const nilu8ptrc: *const u8 = 0 as *const u8;

#[allow(non_camel_case_types)]
pub type wcharptr = *mut libc::wchar_t;
#[allow(non_camel_case_types)]
pub type i8ptr = *mut i8;
#[allow(non_camel_case_types)]
pub type u8ptr = *mut u8;
#[allow(non_camel_case_types)]
pub type wcharptrc = *const libc::wchar_t;
#[allow(non_camel_case_types)]
pub type i8ptrc = *const i8;
#[allow(non_camel_case_types)]
pub type u8ptrc = *const u8;

// tuple struct or tuple variant, found type alias `I8ptr`
// tuple structs
pub struct Voidptrwp(*mut c_void);
pub struct Charptrwp(*mut c_char);
pub struct U8ptrwp(*mut u8);
pub struct I8ptrwp(*mut i8);

impl std::fmt::Display for I8ptrwp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let addr: usize = 0;
        // let x = &self;
        // x as *mut I8ptrwp;
        let p2 = &addr as *const usize;
        let p3 = self as *const I8ptrwp;
        let p4 = p2 as *mut c_void;
        let p5 = p3 as *mut c_void;
        unsafe {
            libc::memcpy(p4, p5, sizeof(addr));
        }
        // println!("cpyaddr 0x{}", addr);

        // f.write_str
        // f.write_char
        // f.write_fmt
        // f.write_i32,i64...
        let _ = f.write_str(format!("0x{}", addr).as_str());

        // println!("fmt *mut i8 0x{}", p3);
        let _ = self.0;
        Ok(())
    }
}
impl std::fmt::Display for U8ptrwp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let addr: usize = 0;
        // let x = &self;
        // x as *mut I8ptrwp;
        let p2 = &addr as *const usize;
        let p3 = self as *const U8ptrwp;
        let p4 = p2 as *mut c_void;
        let p5 = p3 as *mut c_void;
        unsafe {
            libc::memcpy(p4, p5, sizeof(addr));
        }
        // println!("cpyaddr 0x{}", addr);

        // f.write_str
        // f.write_char
        // f.write_fmt
        // f.write_i32,i64...
        let _ = f.write_str(format!("0x{}", addr).as_str());

        // println!("fmt *mut i8 0x{}", p3);
        let _ = self.0;
        Ok(())
    }
}
impl std::fmt::Display for Charptrwp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let addr: usize = 0;
        // let x = &self;
        // x as *mut I8ptrwp;
        let p2 = &addr as *const usize;
        let p3 = self as *const Charptrwp;
        let p4 = p2 as *mut c_void;
        let p5 = p3 as *mut c_void;
        unsafe {
            libc::memcpy(p4, p5, sizeof(addr));
        }
        // println!("cpyaddr 0x{}", addr);

        // f.write_str
        // f.write_char
        // f.write_fmt
        // f.write_i32,i64...
        let _ = f.write_str(format!("0x{}", addr).as_str());

        // println!("fmt *mut i8 0x{}", p3);
        let _ = self.0;
        Ok(())
    }
}
impl std::fmt::Display for Voidptrwp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let addr: usize = 0;
        // let x = &self;
        // x as *mut I8ptrwp;
        let p2 = &addr as *const usize;
        let p3 = self as *const Voidptrwp;
        let p4 = p2 as *mut c_void;
        let p5 = p3 as *mut c_void;
        unsafe {
            libc::memcpy(p4, p5, sizeoft::<usize>());
        }
        // println!("cpyaddr 0x{}", addr);

        // f.write_str
        // f.write_char
        // f.write_fmt
        // f.write_i32,i64...
        let _ = f.write_str(format!("0x{}", addr).as_str());

        // println!("fmt *mut i8 0x{}", p3);
        let _ = self.0;
        Ok(())
    }
}

// impl std::fmt::Display for *const i8 {

// }
// impl std::fmt::Display for *mut u8 {

// }
// impl std::fmt::Display for *const u8 {

// }
// impl std::fmt::Display for *const libc::c_void {

// }
// impl std::fmt::Display for *mut libc::c_void {

// }
