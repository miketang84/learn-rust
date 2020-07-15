use std::slice;

#[no_mangle]
pub extern "C" fn addtwo0(a: u32, b: u32) {
    let c = a + b;
    println!("print in rust, sum is: {}", c);
}

#[no_mangle]
pub extern "C" fn addtwo1(a: u32, b: u32) -> u32 {
    let c = a + b;
    println!("print in rust, sum is: {}", c);
    c
}

#[no_mangle]
pub extern "C" fn sum_of_array(array: *const u32, len: usize) -> u32 {
    assert!(!array.is_null());
    let array = unsafe {
        slice::from_raw_parts(array, len)
    };

    array.iter().sum()
}
