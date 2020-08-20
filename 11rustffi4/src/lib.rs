use std::os::raw::c_int;
use std::slice;

#[repr(C)]
#[derive(Debug)]
pub struct Student {
    pub num: c_int,
    pub total: c_int,
}

#[no_mangle]
pub extern "C" fn fill_students(p_stu: *mut Student, n: c_int) {
    assert!(!p_stu.is_null());
    let s: &mut [Student] = unsafe { slice::from_raw_parts_mut(p_stu, n as usize) };
    for elem in s.iter_mut() {
        // fill any valid values
        elem.num = 1 as c_int;
        elem.total = 100 as c_int;
    }
}

#[no_mangle]
pub extern "C" fn print_students(p_stu: *mut Student, n: c_int) {
    assert!(!p_stu.is_null());
    let s: &[Student] = unsafe { slice::from_raw_parts(p_stu, n as usize) };
    for elem in s.iter() {
        println!("print in rust side: {:?}", elem);
    }
}

