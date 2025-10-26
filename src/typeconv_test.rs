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
fn string2_refstr_t() {
    let result = addint(2, 3);
    let s = format!("str{}", 12345);
    let refs = string2_refstr(&s);
    eprintln!("hehehhe {} {} {}", result, typeofv2(678), refs);
    // l.errprint(1, 2.into());
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
