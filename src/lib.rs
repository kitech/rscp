use std::ffi::CStr;
use std::ffi::CString;
// use std::any;
use once_cell::sync::Lazy;
use std::any::Any;
use std::collections::HashMap;
use std::fmt;

// pub mod dummy; // dont likeuse this
// like this include!
include!("./typedefs.rs");
include!("./typeconv.rs");
include!("./dummy.rs");
include!("./demangle.rs");
include!("./splog.rs");
include!("./ffiparam.rs");

// cargo test --  --nocapture
#[cfg(test)]
mod tests {
    use super::*;
    include!("./tests.rs");
    include!("./typedefs_test.rs");
    include!("./typeconv_test.rs");
    include!("./dummy_test.rs");
    include!("./demangle_test.rs");
    include!("./splog_test.rs");
    include!("./ffiparam_test.rs");
}

// utils make rustc happy
pub fn useit<T>(_vx: T) {}

pub fn initmod() {
    if false {
        initmodinner();
    }
}
fn initmodinner() {
    if false {
        let prm = &ffiparam::default();
        dummy_ffifuncproxy_placeholder(prm);
        ffifuncproxy_rs2go(prm);
    }
}

/////////// globals

// must ptr or &
#[allow(non_upper_case_globals)]
pub static mut globvars: Lazy<HashMap<usize, String>> = Lazy::new(|| {
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

pub fn globvarput2<T>(v: &T) -> usize {
    let p1 = v as *const T;
    let ret = p1 as usize;
    unsafe {
        globvars.insert(ret, "wtval2".into());
    }
    return ret;
}
pub fn globvarput<T>(v: &mut T) -> usize {
    // let x = &HashMap::<usize,usize>::new();
    // let y : usize = x;
    let p1 = v as *const T;
    let ret = p1 as usize;
    unsafe {
        globvars.insert(ret, "wtval".into());
    }
    return ret;
}
pub fn globvarget2<T>(/*v : &mut T,*/ n: usize) -> &'static T {
    let p1 = n as *const T;
    let p2 = p1 as *const T;
    let p3 = unsafe { &*p2 };
    p3
}
pub fn globvarget<T>(/*v : &mut T,*/ n: usize) -> &'static mut T {
    // let p1 : &dyn Any = n as *const c_void;
    let p1 = n as *const T;
    let p2 = p1 as *mut T;
    let p3 = unsafe { &mut *p2 };
    p3
}
pub fn globvardel2<T>(v: &T) -> usize {
    let p1 = v as *const T;
    let ret = p1 as usize;
    unsafe {
        globvars.remove(&ret);
    }
    return ret;
}
pub fn globvardel<T>(/*v : T*/ n: usize) -> bool {
    unsafe {
        globvars.remove(&n);
    }
    true
}
pub fn globvarlen() -> usize {
    unsafe { globvars.len() }
}

/////////// template
#[no_mangle]
pub extern "C" fn addint(left: isize, right: isize) -> isize {
    left + right + addint2(left, right)
}
// #[inline(never)]
pub extern "C" fn addint2(left: isize, right: isize) -> isize {
    left + right
}
