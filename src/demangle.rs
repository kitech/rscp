use rustc_demangle::demangle;

// seems copy two times???
#[no_mangle]
pub extern "C" fn rust_demangle(name: charptrc, ret: voidptr) -> voidptr {
    let x = unsafe { CStr::from_ptr(name) };
    let y = demangle(x.to_str().unwrap()).to_string();
    // println!("{},{}", y, y.len());
    unsafe {
        libc::memcpy(ret, y.as_bytes().as_ptr() as voidptrc, y.len());
    }
    return ret;
}
