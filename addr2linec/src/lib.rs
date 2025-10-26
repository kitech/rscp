use addr2line::{self, Loader};

extern crate libc;
use std::ffi::CStr;

// only support gcc compiled binary
// not support tcc ...
pub fn new_loader(filename: *mut libc::c_char) -> *mut Loader {
    // println!("filename= {}", filename as usize,);

    let c_str = unsafe { CStr::from_ptr(filename) };
    let filename2 = c_str.to_str(); // &str
    let v = addr2line::Loader::new(filename2.unwrap()).unwrap();

    return Box::into_raw(Box::new(v));
}
pub fn drop_loader(obj: *mut Loader) {
    unsafe { drop(Box::from_raw(obj)) }
}
// rebox and return after use, or will dropped, cannot reuse
pub fn find_location(obj: *mut Loader, addr: u64, buf: *mut libc::c_void) -> *mut Loader {
    // println!("{} {} {}", addr, obj as usize, buf as usize);
    let loaderx = unsafe { Box::from_raw(obj) };
    let locval = loaderx.find_location(addr).unwrap().unwrap();
    // println!("{:?} {:?}", locval.file, locval.line);

    unsafe {
        libc::memcpy(
            buf,
            locval.file.unwrap().as_ptr() as *const libc::c_void,
            locval.file.unwrap().len(),
        );
    }
    return Box::into_raw(loaderx);
}

pub fn add(left: u64, right: u64) -> u64 {
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
