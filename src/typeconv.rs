/////////////
// bug: android
pub fn cstrfrom_u8ptr0(ptr: *const i8) -> String {
    let x = unsafe { CStr::from_ptr(ptr) };
    let y = x.to_str().to_owned().unwrap();
    y.to_string()
    // return x.tostr().unwrap()
}
pub fn cstrfrom_u8ptr(ptr: *const u8, len: usize) -> &'static str {
    unsafe {
        let vcc = core::slice::from_raw_parts(ptr, len + 1);
        // println!("xxxx {}", vcc.len());
        let x = CStr::from_bytes_with_nul_unchecked(vcc);
        let xs = x.to_str().to_owned().unwrap();

        return xs;
    }
}
// bug: android
pub fn cstrfrom_u8ptr2(ptr: *const u8, len: usize) -> String {
    unsafe {
        let vcc = core::slice::from_raw_parts(ptr, len + 1);
        // println!("xxxx {}", vcc.len());
        let x = CStr::from_bytes_with_nul_unchecked(vcc);
        let xs: String = x.to_str().unwrap().into();

        return xs;
    }
    // unsafe {
    // let xs = cstrfrom_u8ptr(ptr, len);

    // return xs.to_string()
    // }
}
pub fn cstrfrom_usizeptr(ptrx: usize, len: usize) -> String {
    if ptrx == 0 {
        return "nil".into();
    }

    let ptr = ptrx as *const u8;
    // unsafe {
    let xs = cstrfrom_u8ptr2(ptr, len);

    return xs;
    // }
}

// this works fine for android if ptr is null terminated c string
pub fn cstrfrom_u8ptr3(ptr: *const u8, lenx: usize) -> String {
    unsafe {
        let x = CStr::from_ptr(ptr as *const libc::c_char);
        let xs: String = x.to_str().unwrap().into();
        return xs;
    }
}
pub fn cstrfrom_usizeptr3(ptrx: usize, len: usize) -> String {
    if ptrx == 0 {
        return "nil".into();
    }
    let ptr = ptrx as *const u8;
    // unsafe {
    let xs = cstrfrom_u8ptr3(ptr, len);

    return xs;
    // }
}
// todo
// pub fn CStringfrom_u8ptr(ptrx :*const u8, len: usize) -> CString {
// let vcc  = core::slice::from_raw_parts(ptrx, len+1);
// vcc.into()
// return ()
// }
pub fn refstr2_cstru8(v: &str) -> *const u8 {
    v.as_ptr()
}
pub fn refstr2_cstrusize(v: &str) -> usize {
    v.as_ptr() as usize
}
pub fn refstr2_string(v: &str) -> String {
    v.into()
}
pub fn string2_refstr(v: &String) -> &str {
    let vref = v.as_str();
    return vref;
}

// c memory functions
pub fn cfree_usize(ptrx: usize) {
    let ptr = ptrx as *mut c_void;
    unsafe {
        libc::free(ptr);
    }
}
pub fn cfree_cvoid(ptr: *mut c_void) {
    unsafe {
        libc::free(ptr);
    }
}
pub fn cfree_u8ptr(ptrx: *const u8) {
    let ptr = ptrx as *mut c_void;
    unsafe {
        libc::free(ptr);
    }
}
pub fn cfree_cchar(ptrx: *const c_char) {
    let ptr = ptrx as *mut c_void;
    unsafe {
        libc::free(ptr);
    }
}
// todo

pub fn cfree<T: std::marker::Copy + 'static>(ptrx: T) {
    let tyval = typeofv(&ptrx);
    let tyref = (&tyval).as_str();
    // println!("ptrx type {}", tyref);

    let mut retptr = 0 as *mut c_void;
    _ = retptr;
    let p: &dyn Any = &ptrx;
    p.is::<usize>();
    // <dyn Any>::is::<usize>;
    match p {
        val if p.is::<usize>() => {
            let p2 = val.downcast_ref::<usize>().unwrap();
            let p3 = *p2 as usize as *mut c_void;
            retptr = p3;
            unsafe {
                libc::free(p3);
            }
        }
        val if p.is::<*const core::ffi::c_void>() => {
            let p2 = val.downcast_ref::<*const c_void>().unwrap();
            let p3 = *p2 as *mut c_void;
            retptr = p3;
            unsafe {
                libc::free(p3);
            }
        }
        val if p.is::<*mut core::ffi::c_void>() => {
            let p2 = val.downcast_ref::<*mut c_void>().unwrap();
            let p3 = *p2 as *mut c_void;
            retptr = p3;
            unsafe {
                libc::free(p3);
            }
        }
        val if p.is::<*const u8>() => {
            let p2 = val.downcast_ref::<*const u8>().unwrap();
            let p3 = *p2 as *mut c_void;
            retptr = p3;
            unsafe {
                libc::free(p3);
            }
        }
        val if p.is::<*mut u8>() => {
            let p2 = val.downcast_ref::<*mut u8>().unwrap();
            let p3 = *p2 as *mut c_void;
            retptr = p3;
            unsafe {
                libc::free(p3);
            }
        }
        val if p.is::<*const i8>() => {
            let p2 = val.downcast_ref::<*const i8>().unwrap();
            let p3 = *p2 as *mut c_void;
            retptr = p3;
            unsafe {
                libc::free(p3);
            }
        }
        val if p.is::<*mut i8>() => {
            let p2 = val.downcast_ref::<*mut i8>().unwrap();
            let p3 = *p2 as *mut c_void;
            retptr = p3;
        }
        _ => {
            println!("ptrx type {} {}", tyref, "ptrx");
            todo!()
        }
    }
    unsafe {
        libc::free(retptr);
    }

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
                todo!()
            }
        }
    }
    // let ptr = ptrx as *mut c_void;
    // unsafe { libc::free(ptr); }
}

pub fn ptrtostr2<T: 'static>(ptrx: &T) -> String {
    let p = ptrx as *const T as *const c_void;
    ptrtostr::<*const c_void>(p)
    // "xx".into()
}
pub fn ptrtostr<T: std::marker::Copy + 'static>(ptrx: T) -> String {
    let tyval = typeofv(&ptrx);
    let tyref = (&tyval).as_str();
    // println!("ptrx type {}", tyref);

    let mut retval: usize = 0;
    useit(retval);
    let p: &dyn Any = &ptrx;
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
            let p3 = *p2 as usize;
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
            return format!("{}", p3);
        }
        val if p.is::<u32>() => {
            let p2 = val.downcast_ref::<u32>().unwrap();
            let p3 = *p2 as usize;
            retval = p3;
        }
        val if p.is::<i64>() => {
            let p2 = val.downcast_ref::<i32>().unwrap();
            let p3 = *p2 as i64;
            return format!("{}", p3);
        }
        val if p.is::<u64>() => {
            let p2 = val.downcast_ref::<u64>().unwrap();
            let p3 = *p2 as u64;
            return format!("{}", p3);
        }
        val if p.is::<bool>() => {
            let p2 = val.downcast_ref::<bool>().unwrap();
            // let p3 = *p2 as u64;
            return format!("{}", p2);
        }
        val if p.is::<f32>() => {
            let p2 = val.downcast_ref::<f32>().unwrap();
            // let p3 = *p2 as u64;
            return format!("{}", p2);
        }
        val if p.is::<f64>() => {
            let p2 = val.downcast_ref::<f64>().unwrap();
            // let p3 = *p2 as u64;
            return format!("{}", p2);
        }

        _ => {
            // fn()
            let mut t = tyref.chars();
            let b0 = t.nth(0).unwrap() == 'f';
            t = tyref.chars();
            let b1 = t.nth(1).unwrap() == 'n';
            t = tyref.chars();
            let b2 = t.nth(2).unwrap() == '(';
            // println!("len {} {} {}", b0,b1,b2);
            if tyref.len() > 2 && b0 && b1 && b2 {
                let p2 = (&ptrx) as *const T as *const c_void;
                let p3 = &retval as *const usize as *mut c_void;
                unsafe {
                    libc::memcpy(p3, p2, sizeof(retval));
                }
                // println!("len222: {} {}", tyref.len(), tyref);
            } else {
                println!("ptrx type {} {}", tyref, "ptrx");
                todo!()
            }
        }
    }

    let retstr = format!("0x{}", retval);
    retstr.into()
}

/////////// types
// usage: typeofv(var)
pub fn typeofv<T>(_vx: &T) -> String {
    std::any::type_name::<T>().into()
}
// usage: typeofv2(literal)
pub fn typeofv2<T>(_vx: T) -> String {
    std::any::type_name::<T>().into()
}

pub fn lenof<T>(_vx: &T) -> usize {
    0
}
pub fn lenof2<T>(_vx: T) -> usize {
    0
}

// usage: sizeoft::<i32>()
pub fn sizeoft<T>() -> usize {
    std::mem::size_of::<T>()
}
// usage: sizeof(var)
pub fn sizeof<T>(_vx: T) -> usize {
    std::mem::size_of::<T>()
}
