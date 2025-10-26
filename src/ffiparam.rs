// cross langs
// the trait `std::default::Default` is implemented for `u8`
#[derive(Default)]
#[repr(C)]
pub struct ffiparam {
    // ptr: *const i8,
    pub ptr: usize,
    pub len: usize,

    // resp: *const i8,
    pub resp: usize,
    pub len2: usize,

    // errmsg: *const i8
    pub errmsg: usize,
    pub code: usize,
}

impl ffiparam {
    pub fn tostr(&self) -> String {
        format!(
            "ffiparam: p1:{} s1:{} p2:{} s2:{} p3:{} s3:{}",
            self.ptr, self.len, self.resp, self.len2, self.errmsg, self.code
        )
    }

    pub fn setdata<T: 'static>(&mut self, data: T) {
        // println!("xxx {}", typeofv::<T>(&data));
        let p: &dyn Any = &data;

        match p {
            val if p.is::<&str>() => {
                let p2 = val.downcast_ref::<&str>().unwrap();
                //println!("xxx &str {} {}", p2.len(), p2);
                self.len = p2.len();
                self.ptr = p2.as_ptr() as usize;
            }
            val if p.is::<String>() => {
                let p2 = val.downcast_ref::<String>().unwrap();
                // println!("xxx String {} {}", p2.len(), p2);
                self.len = p2.len();
                self.ptr = p2.as_ptr() as usize;
            }
            val if p.is::<CString>() => {
                let p2 = val.downcast_ref::<CString>().unwrap();
                let p3 = p2.as_bytes();
                // let p4 : &str = p2.as_str();
                // println!("xxx CString {} {}", p3.len(), p3.len());
                self.len = p3.len();
                self.ptr = p3.as_ptr() as usize;
            }
            // val if p.is::<CStr>() => {
            //     todo!()
            // let p2 = val.downcast_ref::<CStr>().unwrap();
            // let p3 = p2.as_bytes();
            // let p4 : &str = p2.as_str();
            // println!("xxx CString {} {}", p2.len(), p2.len());
            // }
            _ => {
                todo!()
            }
        }
    }

    pub fn getdata1(&self) -> &str {
        ""
    }
    pub fn getdata2(&self) -> String {
        cstrfrom_usizeptr(self.resp, self.len2)
    }
    // pub fn getdata3(&self) -> CStr {
    //     ""
    // }
}

fn dummy_ffifuncproxy_placeholder(_v: &ffiparam) {}
#[allow(non_upper_case_globals)]
static mut ffifuncproxy_rs2go_fnptr: fn(_v: &ffiparam) = dummy_ffifuncproxy_placeholder;

pub fn ffifuncproxy_rs2go(_v: &ffiparam) {
    unsafe {
        ffifuncproxy_rs2go_fnptr(_v);
    }
}
