#[test]
fn test_string_to_cptr() {
    let s = String::from("hello");
    let ptr = string_to_cptr(s);
    let result = unsafe { cptr_to_string(ptr) };
    assert_eq!(result, "hello");
    free_cptr(ptr);
}

#[test]
fn test_str_to_cptr() {
    let s = "hello";
    let ptr = str_to_cptr(s);
    let result = unsafe { cptr_to_string(ptr) };
    assert_eq!(result, s);
    free_cptr(ptr);
}

#[test]
fn test_cstr_to_cptr() {
    let cstr = CStr::from_bytes_with_nul(b"test\0").unwrap();
    let ptr = cstr_to_cptr(cstr);
    let result = unsafe { cptr_to_string(ptr) };
    assert_eq!(result, "test");
    free_cptr(ptr);
}

#[test]
fn test_cstring_to_cptr() {
    let cs = CString::new("test").unwrap();
    let ptr = cstring_to_cptr(&cs);
    let recovered = unsafe { CStr::from_ptr(ptr) };
    assert_eq!(cs.as_c_str(), recovered);
}

#[test]
fn test_u8ptr_to_string() {
    let s = "hello";
    let result = u8ptr_to_string(s.as_ptr(), s.len());
    assert_eq!(result, s);
}

#[test]
fn test_u8ptr_to_vec() {
    let v = b"hello".to_vec();
    let result = u8ptr_to_vec(v.as_ptr(), v.len());
    assert_eq!(result, v);
}

#[test]
fn test_path_to_cptr() {
    let p = Path::new("/tmp/test");
    let ptr = path_to_cptr(p);
    let result = unsafe { cptr_to_string(ptr) };
    assert_eq!(result, "/tmp/test");
    free_cptr(ptr);
}

#[test]
fn test_string_vec_roundtrip() {
    let s = String::from("hello");
    let v = string_to_vec(&s);
    assert_eq!(v, b"hello");
    let back = vec_to_string(v);
    assert_eq!(back, s);
}

#[test]
fn test_unicode_cptr() {
    let s = "中文测试";
    let ptr = str_to_cptr(s);
    let result = unsafe { cptr_to_string(ptr) };
    assert_eq!(result, s);
    free_cptr(ptr);
}

#[test]
#[should_panic]
fn test_invalid_utf8_panics() {
    let v = vec![0xFF, 0xFE];
    let _ = vec_to_string(v);
}

#[test]
fn test_u8ptr_to_cstring() {
    let cs = CString::new("null terminated").unwrap();
    let ptr = cs.as_ptr() as *const u8;
    let result = u8ptr_to_cstring(ptr);
    assert_eq!(result.to_str().unwrap(), "null terminated");
}

#[test]
fn test_u8ptr_to_cstr() {
    let cs = CString::new("cstr").unwrap();
    let ptr = cs.as_ptr() as *const u8;
    let result = u8ptr_to_cstr(ptr);
    assert_eq!(result.to_str().unwrap(), "cstr");
}

#[test]
fn test_u8ptr_to_str() {
    let cs = CString::new("str").unwrap();
    let ptr = cs.as_ptr() as *const u8;
    let result = u8ptr_to_str(ptr);
    assert_eq!(result, "str");
}

#[test]
fn test_u8ptr_to_string_copy() {
    let cs = CString::new("copy string").unwrap();
    let ptr = cs.as_ptr() as *const u8;
    let result = u8ptr_to_string_copy(ptr);
    assert_eq!(result, "copy string");
}
