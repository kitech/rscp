#[test]
fn log_line() {
    let l = Splog::default();
    let result = addint2(2, 3);
    // eprintln!("hehehhe {} {}", result, typeofv(678))
    // l.errprint(1, 2.into());
    // use core::fmt::rt::Argument;
    // let args = fmt::Arguments::new_v1();
    // l.print(123, args);

    useit(l);
    useit(result);
}
