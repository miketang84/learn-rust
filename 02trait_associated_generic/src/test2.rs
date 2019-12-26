pub trait Converter<T=i32> {
    fn convert(&self) -> T;
}

struct MyInt;

impl Converter for MyInt {
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

    let output: i32 = my_int.convert();
    println!("output is: {}", output);

    let output: f32 = my_int.convert();
    println!("output is: {}", output);

}
