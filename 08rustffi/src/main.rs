use std::os::raw::{c_int, c_float};
use std::ffi::CString;
use std::slice;

#[repr(C)]
#[derive(Debug)]
pub struct Student {
    pub num: c_int,
    pub total: c_int,
    pub name: [u8; 20],
    pub scores: [c_float; 3],
}

#[link(name = "cfoo")]
extern "C" {
    fn create_students(n: c_int) -> *mut Student;
    fn print_students(p_stu: *mut Student, n: c_int);
    fn release_students(p_stu: *mut Student);
}


fn main() {
    let n = 3;
    unsafe {
        let p_stu = create_students(n as c_int);
        assert!(!p_stu.is_null());

        let s: &mut [Student] = slice::from_raw_parts_mut(p_stu, n as usize);
        for elem in s.iter_mut() {
            elem.num = 1 as c_int;
            elem.total = 100 as c_int;

            let c_string = CString::new("Mike").expect("CString::new failed");
            let bytes = c_string.as_bytes_with_nul();
            elem.name[..bytes.len()].copy_from_slice(bytes);

            elem.scores = [30.0 as c_float, 40.0 as c_float, 30.0 as c_float];
        }

        println!("rust side print: {:?}", s);

        print_students(p_stu, n as c_int);


        release_students(p_stu);
    }
    
    println!("Over.");
}

