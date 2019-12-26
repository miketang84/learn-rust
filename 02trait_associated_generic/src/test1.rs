pub trait Converter<T> {
    fn convert(&self) -> T;
}

struct MyInt;

impl Converter<i32> for MyInt {
    fn convert(&self) -> i32 {
        42
    }
}

impl Converter<f32> for MyInt {
    fn convert(&self) -> f32 {
        52.0
    }
}


fn main() {
    let my_int = MyInt;

    // Error: could not use turbofish syntax here
    // let output = my_int.convert::<i32>();
    let output: i32 = my_int.convert();
    println!("output is: {}", output);

    // Error: could not use turbofish syntax here
    // let output = my_int.convert::<f32>();
    let output: f32 = my_int.convert();
    println!("output is: {}", output);

}
