use std::os::raw::c_int;
use std::ffi::c_void;

pub type SumSquareCB = unsafe extern fn(c_int, *mut c_void);

#[link(name = "ccode02")]
extern {
    pub fn sum_square_cb02(a: c_int, b: c_int, cb: SumSquareCB, user_data: *mut c_void);
}

pub unsafe extern fn cb_func(result: c_int, user_data: *mut c_void) {
    let data = &mut *(user_data as *mut c_int);
    *data += result;
}

fn main() {
    let mut sum = 0;

    unsafe {
        sum_square_cb02(
            3,
            4,
            cb_func,
            &mut sum as *mut c_int as *mut c_void);
    }

    println!("The sum is {}", sum);
}
