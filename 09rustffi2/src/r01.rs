use std::os::raw::c_int;

pub type SumSquareCB = unsafe extern fn(c_int);

#[link(name = "ccode01")]
extern {
    pub fn sum_square_cb01(a: c_int, b: c_int, cb: SumSquareCB);
}

pub unsafe extern fn cb_func(result: c_int) {
    println!("The result in callback function is: {}", result);
}

fn main() {
    unsafe {
        sum_square_cb01(3, 4, cb_func);
    }
}
