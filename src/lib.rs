
extern crate libc;
use libc::{c_void, c_char}; //, malloc, free};
use std::ffi::CStr;
use std::ffi::CString;
// use std::any;
use std::fmt;
use std::any::Any;
use std::collections::HashMap;
use once_cell::sync::Lazy;

// utils make rustc happy
pub fn useit<T>(_vx : T) {  }

// cross langs
// the trait `std::default::Default` is implemented for `u8`
#[derive(Default)]
#[repr(C)]
pub struct ffiparam {

    // ptr: *const i8,
    pub ptr: usize,
    pub len: usize,

    // resp: *const i8,
    pub resp: usize,
    pub len2: usize,

    // errmsg: *const i8
    pub errmsg: usize,
    pub code: usize,

}

impl ffiparam {

    pub fn setdata<T: 'static >(&mut self, data : T) {
        // println!("xxx {}", typeofv::<T>(&data));
        let p : &dyn Any = &data;
        
        match p {
            val if p.is::<&str>() => {
                
                let p2 = val.downcast_ref::<&str>().unwrap();
                //println!("xxx &str {} {}", p2.len(), p2);
                self.len = p2.len();
                self.ptr = p2.as_ptr() as usize;
            }
            val if p.is::<String>() => {
                let p2 = val.downcast_ref::<String>().unwrap();
                // println!("xxx String {} {}", p2.len(), p2);
                self.len = p2.len();
                self.ptr = p2.as_ptr() as usize;
            }
            val if p.is::<CString>() => {
                let p2 = val.downcast_ref::<CString>().unwrap();
                let p3 = p2.as_bytes();
                // let p4 : &str = p2.as_str();
                // println!("xxx CString {} {}", p3.len(), p3.len());
                self.len = p3.len();
                self.ptr = p3.as_ptr() as usize;
            }
            // val if p.is::<CStr>() => {
            //     todo!()
                // let p2 = val.downcast_ref::<CStr>().unwrap();
                // let p3 = p2.as_bytes();
                // let p4 : &str = p2.as_str();
                // println!("xxx CString {} {}", p2.len(), p2.len());
            // }
            _ => {
                todo!()
            }
        }
    }

    pub fn getdata1(&self) -> &str {
        ""
    }
    pub fn getdata2(&self) -> String {
        cstrfrom_usizeptr(self.resp, self.len2)
    }
    // pub fn getdata3(&self) -> CStr {
    //     ""
    // }
}

fn dummy_ffifuncproxy_placeholder(_v: &ffiparam) {}
#[allow(non_upper_case_globals)]
static mut ffifuncproxy_rs2go_fnptr : fn(_v:&ffiparam) = dummy_ffifuncproxy_placeholder;

pub fn ffifuncproxy_rs2go(_v:&ffiparam) {
    unsafe {
        ffifuncproxy_rs2go_fnptr(_v);
    }
}

pub fn initmod() {

    if false {
        initmodinner();
    }
}
fn initmodinner() {
    if false {
        let prm =  &ffiparam::default();
        dummy_ffifuncproxy_placeholder(prm);
        ffifuncproxy_rs2go(prm);
    }
}

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
pub fn refstr2_cstru8(v : &str) -> *const u8 { v.as_ptr() }
pub fn refstr2_cstrusize(v : &str) -> usize { v.as_ptr() as usize }
pub fn refstr2_string(v : &str) -> String { v.into() }
pub fn string2_refstr(v : &String) -> &str {
    let vref  = v.as_str ();
    return vref;
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

pub fn cfree<T: std::marker::Copy + 'static>(ptrx : T) {
    let tyval = typeofv(&ptrx);
    let tyref = (&tyval).as_str();
    // println!("ptrx type {}", tyref);

    let mut retptr = 0 as *mut c_void;
    _ = retptr;
    let p : &dyn Any = & ptrx;
    p.is::<usize>();
    // <dyn Any>::is::<usize>;
    match p {
        val if p.is::<usize>() => {
            let p2 = val.downcast_ref::<usize>().unwrap();
            let p3 = *p2 as usize as  *mut c_void;
            retptr = p3;
             unsafe { libc::free(p3); }
            }
        val if p.is::<*const core::ffi::c_void>() => {
            let p2 = val.downcast_ref::<*const c_void>().unwrap();
            let p3 = *p2 as  *mut c_void;
            retptr = p3;
            unsafe { libc::free(p3); }
        }
        val if p.is::<*mut core::ffi::c_void>() => {
            let p2 = val.downcast_ref::<*mut c_void>().unwrap();
            let p3 = *p2 as  *mut c_void;
            retptr = p3;
            unsafe { libc::free(p3); }
        }
        val if p.is::<*const u8>() => {
            let p2 = val.downcast_ref::<*const u8>().unwrap();
            let p3 = *p2 as  *mut c_void;
            retptr = p3;
            unsafe { libc::free(p3); }
        }
        val if p.is::<*mut u8>() => {
            let p2 = val.downcast_ref::<*mut u8>().unwrap();
            let p3 = *p2 as  *mut c_void;
            retptr = p3;
            unsafe { libc::free(p3); }
        }
        val if p.is::<*const i8>() => {
            let p2 = val.downcast_ref::<*const i8>().unwrap();
            let p3 = *p2 as  *mut c_void;
            retptr = p3;
            unsafe { libc::free(p3); }
        }
        val if p.is::<*mut i8>() => {
            let p2 = val.downcast_ref::<*mut i8>().unwrap();
            let p3 = *p2 as  *mut c_void;
            retptr = p3;
            
        }
        _ => {
            println!("ptrx type {} {}", tyref, "ptrx");
             todo!()}
    }
    unsafe { libc::free(retptr); }

    if false {
    match tyref {
        "usize" => {
            // let ptr =  <dyn Any>::is::usize(ptrx) ;
            // libc::free(ptr);
            // ptrx.downcast::<usize>().unwrap();
        }
        "*const core::ffi::c_void" => {}
        "*const i8" => {}
        "*const u8" => {}
        "*mut core::ffi::c_void" => {}
        "*mut i8" => {}
        "*mut u8" => {}
        &_ => { 
            println!("ptrx type {}", tyref);
            todo!()}
    }
    }
    // let ptr = ptrx as *mut c_void;
    // unsafe { libc::free(ptr); }
}

pub fn ptrtostr2<T: 'static>(ptrx :& T) -> String {
    let p = ptrx as *const T as *const c_void;
    ptrtostr::<*const c_void>(p)
    // "xx".into()
}
pub fn ptrtostr<T: std::marker::Copy + 'static>(ptrx : T) -> String {
    let tyval = typeofv(&ptrx);
    let tyref = (&tyval).as_str();
    // println!("ptrx type {}", tyref);

    let mut retval : usize = 0;
    useit(retval);
    let p : &dyn Any = & ptrx;
    // p.is::<usize>();
    // <dyn Any>::is::<usize>;
    match p {
        val if p.is::<usize>() => {
            let p2 = val.downcast_ref::<usize>().unwrap();
            retval = *p2;
            }
        val if p.is::<*const core::ffi::c_void>() => {
            let p2 = val.downcast_ref::<*const c_void>().unwrap();
            let p3 = *p2 as usize;
            retval = p3;
        }
        val if p.is::<*mut core::ffi::c_void>() => {
            let p2 = val.downcast_ref::<*mut c_void>().unwrap();
            let p3 = *p2 as usize;
            retval = p3;
        }
        val if p.is::<*const u8>() => {
            let p2 = val.downcast_ref::<*const u8>().unwrap();
            let p3 = *p2 as usize;
            retval = p3;
        }
        val if p.is::<*mut u8>() => {
            let p2 = val.downcast_ref::<*mut u8>().unwrap();
            let p3 = *p2 as  usize;
            retval = p3;
        }
        val if p.is::<*const i8>() => {
            let p2 = val.downcast_ref::<*const i8>().unwrap();
            let p3 = *p2 as usize;
            retval = p3;
        }
        val if p.is::<*mut i8>() => {
            let p2 = val.downcast_ref::<*mut i8>().unwrap();
            let p3 = *p2 as usize;
            retval = p3;
        }
        val if p.is::<i32>() => {
            let p2 = val.downcast_ref::<i32>().unwrap();
            let p3 = *p2 as i32;
            return format!("{}", p3)
        }
        val if p.is::<u32>() => {
            let p2 = val.downcast_ref::<u32>().unwrap();
            let p3 = *p2 as usize;
            retval = p3;
        }
        val if p.is::<i64>() => {
            let p2 = val.downcast_ref::<i32>().unwrap();
            let p3 = *p2 as i64;
            return format!("{}", p3)
        }
        val if p.is::<u64>() => {
            let p2 = val.downcast_ref::<u64>().unwrap();
            let p3 = *p2 as u64;
            return format!("{}", p3)
        }
        val if p.is::<bool>() => {
            let p2 = val.downcast_ref::<bool>().unwrap();
            // let p3 = *p2 as u64;
            return format!("{}", p2)
        }
        val if p.is::<f32>() => {
            let p2 = val.downcast_ref::<f32>().unwrap();
            // let p3 = *p2 as u64;
            return format!("{}", p2)
        }
        val if p.is::<f64>() => {
            let p2 = val.downcast_ref::<f64>().unwrap();
            // let p3 = *p2 as u64;
            return format!("{}", p2)
        }

        _ => {
            // fn()
            let mut t = tyref.chars();
            let b0 = t.nth(0).unwrap() == 'f' ;
            t = tyref.chars();
            let b1 = t.nth(1).unwrap() == 'n' ;
            t = tyref.chars();
            let b2 = t.nth(2).unwrap() == '(' ;
            // println!("len {} {} {}", b0,b1,b2);
            if tyref.len() > 2 && b0 && b1 && b2 {
                let p2 = (&ptrx) as *const T as *const c_void;
                let p3 = &retval as *const usize as *mut c_void;
                unsafe {
                    libc::memcpy(p3, p2, sizeof(retval));
                }
                // println!("len222: {} {}", tyref.len(), tyref);
            }else{
            println!("ptrx type {} {}", tyref, "ptrx");
            todo!()
            }
        }
    }

    let retstr = format!("0x{}", retval);
    retstr.into()
}

/////////// log
// todo
// usage:
#[derive(Default)]
pub struct Splog {

}

impl Splog {
    pub fn print<T>(&self, vx : T, args : fmt::Arguments) {
        useit(vx);
        useit(args);
    }

    pub fn errprint<T>(&self, vx : T, args : fmt::Arguments) {
        useit(vx);
        useit(args);

    }
}

impl Splog {
    pub fn nilprint<T>(&self, vx : T, args : fmt::Arguments) {
        useit(vx);
        useit(args);
    
    }
}

/////////// types 
// usage: typeofv(var)
pub fn typeofv<T>(_vx : &T) -> String {
    std::any::type_name::<T>().into()
}
// usage: typeofv2(literal)
pub fn typeofv2<T>(_vx : T) -> String {
    std::any::type_name::<T>().into()
}

pub fn lenof<T>(_vx : &T) -> usize {
    0
}
pub fn lenof2<T>(_vx : T) -> usize {
    0
}

// usage: sizeoft::<i32>()
pub fn sizeoft<T>() -> usize {
    std::mem::size_of::<T>()
}
// usage: sizeof(var)
pub fn sizeof<T>(_vx : T) -> usize {
    std::mem::size_of::<T>()
}


// \see std::ptr::null_mut()
pub const NILVOIDPTRM : *mut c_void = 0 as *mut c_void;
pub const NILCHARPTRM : *mut c_char = 0 as *mut c_char;
pub const NILWCHARPTRM : *mut libc::wchar_t = 0 as *mut libc::wchar_t;
pub const NILI8PTRM : *mut i8 = 0 as *mut i8;
pub const NILU8PTRM : *mut u8 = 0 as *mut u8;
pub const NILVOIDPTR : *const c_void = 0 as *const c_void;
pub const NILCHARPTR : *const c_char = 0 as *const c_char;
pub const NILI8PTR : *const i8 = 0 as *const i8;
pub const NILU8PTR : *const u8 = 0 as *const u8;


pub type Voidptrm = *mut c_void;
pub type Charptrm = *mut c_char;
pub type Wcharptrm = *mut libc::wchar_t;
pub type I8ptrm = *mut i8;
pub type U8ptrm = *mut u8;
pub type Voidptr = *const c_void;
pub type Charptr = *const c_char;
pub type Wcharptr = *const libc::wchar_t;
pub type I8ptr = *const i8;
pub type U8ptr = *const u8;

// tuple struct or tuple variant, found type alias `I8ptr`
// tuple structs
pub struct Voidptrwp(*mut c_void);
pub struct Charptrwp(*mut c_char);
pub struct U8ptrwp(*mut u8);
pub struct I8ptrwp(*mut i8);

impl std::fmt::Display for I8ptrwp {
    fn fmt(&self, f : &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let addr : usize = 0;
        // let x = &self;
        // x as *mut I8ptrwp;
        let p2 = &addr as *const usize;
        let p3 = self as *const I8ptrwp;
        let p4 = p2 as *mut c_void;
        let p5 = p3 as *mut c_void;
        unsafe { libc::memcpy(p4, p5, sizeof(addr)); }
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
    fn fmt(&self, f : &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let addr : usize = 0;
        // let x = &self;
        // x as *mut I8ptrwp;
        let p2 = &addr as *const usize;
        let p3 = self as *const U8ptrwp;
        let p4 = p2 as *mut c_void;
        let p5 = p3 as *mut c_void;
        unsafe { libc::memcpy(p4, p5, sizeof(addr)); }
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
    fn fmt(&self, f : &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let addr : usize = 0;
        // let x = &self;
        // x as *mut I8ptrwp;
        let p2 = &addr as *const usize;
        let p3 = self as *const Charptrwp;
        let p4 = p2 as *mut c_void;
        let p5 = p3 as *mut c_void;
        unsafe { libc::memcpy(p4, p5, sizeof(addr)); }
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
    fn fmt(&self, f : &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let addr : usize = 0;
        // let x = &self;
        // x as *mut I8ptrwp;
        let p2 = &addr as *const usize;
        let p3 = self as *const Voidptrwp;
        let p4 = p2 as *mut c_void;
        let p5 = p3 as *mut c_void;
        unsafe { libc::memcpy(p4, p5, sizeoft::<usize>()); }
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

/////////// globals

// must ptr or &
#[allow(non_upper_case_globals)]
pub static mut globvars : Lazy<HashMap<usize, String>> = Lazy::new(|| {
    // println!("initializing");
    let mut m = HashMap::new();
    if false {
        m.insert(13, "Spica".to_string());
        m.insert(74, "Hoyten".to_string());
        println!("htcnt {}", m.len());
        m.remove(&13);
        m.remove(&74);
    }
    m
});

pub fn globvarput2<T>(v : & T) -> usize {
    let p1 = v as *const T;
    let ret = p1 as usize;
    unsafe { globvars.insert(ret, "wtval2".into()); }
    return ret;
}
pub fn globvarput<T>(v : &mut T) -> usize {
    // let x = &HashMap::<usize,usize>::new();
    // let y : usize = x;
    let p1 = v as *const T;
    let ret = p1 as usize;
    unsafe { globvars.insert(ret, "wtval".into()); }
    return ret;
}
pub fn globvarget2<T>(/*v : &mut T,*/ n : usize) -> &'static T {
    let p1 = n as *const T;
    let p2 = p1 as *const T;
    let p3 = unsafe { & *p2 };
    p3
}
pub fn globvarget<T>(/*v : &mut T,*/ n : usize) -> &'static mut T {
    // let p1 : &dyn Any = n as *const c_void;
    let p1 = n as *const T;
    let p2 = p1 as *mut T;
    let p3 = unsafe { &mut *p2 };
    p3
}
pub fn globvardel2<T>(v : & T) -> usize {
    let p1 = v as *const T;
    let ret = p1 as usize;
    unsafe { globvars.remove(&ret); }
    return ret;
}
pub fn globvardel<T>(/*v : T*/ n : usize) -> bool {
    unsafe { globvars.remove(&n); }
    true
}
pub fn globvarlen() -> usize {
    unsafe { globvars.len() }
}

/////////// template
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// cargo test --  --nocapture
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn cfree_t() {
        let p1 = 0 as usize;
        cfree(p1);
        let p2 = p1 as *const c_void;
        cfree(p2);
        let p22 = p1 as *mut c_void;
        cfree(p22);
        let p3 = p1 as *const c_char;
        cfree(p3);
        let p32 = p1 as *mut c_char;
        cfree(p32);
        let p4 = p1 as *const u8;
        cfree(p4);
        let p42 = p1 as *mut u8;
        cfree(p42);
        let p5 = p1 as *const i8;
        cfree(p5);
        let p52 = p1 as *mut i8;
        cfree(p52);
    }

    #[test]
    fn log_line() {
        let l = Splog::default();
        let result = add(2, 2);
        // eprintln!("hehehhe {} {}", result, typeofv(678))
        // l.errprint(1, 2.into());
        // use core::fmt::rt::Argument;
        // let args = fmt::Arguments::new_v1();
        // l.print(123, args);

        useit(l);
        useit(result);
    }

    #[test]
    fn string2_refstr_t() {
        let result = add(2, 2);
        let s = format!("str{}", 12345);
        let refs = string2_refstr(&s);
        eprintln!("hehehhe {} {} {}", result, typeofv2(678), refs);
        // l.errprint(1, 2.into());
        
    }

    #[test]
    fn ffiparam_setdata() {
        let mut prm = ffiparam::default();
        prm.setdata("ffffff");
        prm.setdata::<String>("ggggggg".into());
        let x = CString::new(b"hello...").unwrap();
        prm.setdata( x );
    }

    #[test]
    fn ptrtostr_test() {
        assert_eq!(ptrtostr(true), "true");
        assert_eq!(ptrtostr(false), "false");
        assert_eq!(ptrtostr(2345), "2345");
        assert_eq!(ptrtostr(-2345), "-2345");
        assert_eq!(ptrtostr(2345 as usize), "0x2345");
        assert_eq!(ptrtostr(2345.678), "2345.678");
        assert_eq!(ptrtostr(2345.678 as f32), "2345.678");
    }

    #[test]
    fn fmti8ptr_test() {
        let ptr = 123 as *mut i8;
        let ptr2 = I8ptrwp(ptr);
        let res = format!("{}", ptr2);
        assert_eq!(res, "0x123");
        {
            let res = format!("{}", I8ptrwp(ptr));
            assert_eq!(res, "0x123");
        }
    }
}


