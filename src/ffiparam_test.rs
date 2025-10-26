#[test]
fn ffiparam_setdata() {
    let mut prm = ffiparam::default();
    prm.setdata("ffffff");
    prm.setdata::<String>("ggggggg".into());
    let x = CString::new(b"hello...").unwrap();
    prm.setdata(x);
}
