pub trait Converter<T=i32> {
    type Output;

    fn convert(&self) -> (Self::Output, T);
}

struct MyInt;

impl Converter for MyInt {
    type Output = i32;
    
    fn convert(&self) -> (Self::Output, i32) {
        (42, 42)
    }
}

impl Converter<f32> for MyInt {
    type Output = i32;
    
    fn convert(&self) -> (Self::Output, f32) {
        (52, 52.0)
    }
}


fn main() {
    let my_int = MyInt;

    let output: (i32, i32) = my_int.convert();
    println!("output is: {:?}", output);

    let output: (i32, f32) = my_int.convert();
    println!("output is: {:?}", output);

}
