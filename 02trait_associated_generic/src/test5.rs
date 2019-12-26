pub trait Converter<T=Self> {
    type Output;

    fn convert(&self) -> (Self::Output, T);
}

#[derive(Debug, Copy, Clone)]
struct MyInt(i32);

impl Converter for MyInt {
    type Output = Self;
    
    fn convert(&self) -> (Self::Output, Self) {
        (*self, *self)
    }
}

impl Converter<f32> for MyInt {
    type Output = Self;
    
    fn convert(&self) -> (Self::Output, f32) {
        (*self, 52.0)
    }
}


fn main() {
    let my_int = MyInt(42);

    let output: (MyInt, MyInt) = my_int.convert();
    println!("output is: {:?}", output);

    let output: (MyInt, f32) = my_int.convert();
    println!("output is: {:?}", output);

}
