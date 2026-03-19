use std::path::Path;

/// String → C 字符串指针 (需 free_cstring_ptr 释放)
pub fn string_to_cptr(s: String) -> *mut std::ffi::c_char {
    CString::new(s).unwrap().into_raw()
}

/// 释放 string_to_cptr 分配的内存
pub fn free_cptr(ptr: *mut std::ffi::c_char) {
    unsafe { CString::from_raw(ptr) };
}

/// C 字符串指针 → String (unsafe, 需保证有效指针)
pub unsafe fn cptr_to_string(ptr: *const std::ffi::c_char) -> String {
    CStr::from_ptr(ptr).to_str().unwrap().to_string()
}

/// &str → C 字符串指针 (需 free_cptr 释放)
pub fn str_to_cptr(s: &str) -> *mut std::ffi::c_char {
    CString::new(s).unwrap().into_raw()
}

/// &CStr → C 字符串指针 (需 free_cptr 释放)
pub fn cstr_to_cptr(cs: &CStr) -> *mut std::ffi::c_char {
    cs.to_owned().into_raw()
}

/// CString → C 字符串指针
pub fn cstring_to_cptr(cs: &CString) -> *const std::ffi::c_char {
    cs.as_ptr()
}

/// u8 指针 + 长度 → String
pub fn u8ptr_to_string(ptr: *const u8, len: usize) -> String {
    unsafe {
        let slice = std::slice::from_raw_parts(ptr, len);
        String::from_utf8(slice.to_vec()).unwrap()
    }
}

/// u8 指针 + 长度 → Vec<u8>
pub fn u8ptr_to_vec(ptr: *const u8, len: usize) -> Vec<u8> {
    unsafe { std::slice::from_raw_parts(ptr, len).to_vec() }
}

/// Path → C 字符串指针 (需 free_cptr 释放)
pub fn path_to_cptr(p: &Path) -> *mut std::ffi::c_char {
    str_to_cptr(p.to_str().unwrap())
}

/// String → Vec<u8> (UTF-8 字节)
pub fn string_to_vec(s: &String) -> Vec<u8> {
    s.as_bytes().to_vec()
}

/// Vec<u8> → String (invalid UTF-8 会 panic)
pub fn vec_to_string(v: Vec<u8>) -> String {
    String::from_utf8(v).unwrap()
}

/// *const u8 (null 结尾) → CString (拷贝)
pub fn u8ptr_to_cstring(ptr: *const u8) -> CString {
    unsafe { CStr::from_ptr(ptr as *const std::ffi::c_char).to_owned() }
}

/// *const u8 (null 结尾) → &CStr
pub fn u8ptr_to_cstr(ptr: *const u8) -> &'static CStr {
    unsafe { CStr::from_ptr(ptr as *const std::ffi::c_char) }
}

/// *const u8 (null 结尾) → String (拷贝)
pub fn u8ptr_to_string_copy(ptr: *const u8) -> String {
    unsafe {
        CStr::from_ptr(ptr as *const std::ffi::c_char)
            .to_str()
            .unwrap()
            .to_string()
    }
}

/// *const u8 (null 结尾) → &str
pub fn u8ptr_to_str(ptr: *const u8) -> &'static str {
    unsafe {
        CStr::from_ptr(ptr as *const std::ffi::c_char)
            .to_str()
            .unwrap()
    }
}
