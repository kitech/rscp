/////////// log
// todo
// usage:
#[derive(Default)]
pub struct Splog {}

impl Splog {
    pub fn print<T>(&self, vx: T, args: fmt::Arguments) {
        useit(vx);
        useit(args);
    }

    pub fn errprint<T>(&self, vx: T, args: fmt::Arguments) {
        useit(vx);
        useit(args);
    }
}

impl Splog {
    pub fn nilprint<T>(&self, vx: T, args: fmt::Arguments) {
        useit(vx);
        useit(args);
    }
}
