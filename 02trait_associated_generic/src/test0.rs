pub trait Converter {
    type Output;

    fn convert(&self) -> Self::Output;
}

struct MyInt;

impl Converter for MyInt {
    type Output = i32;
    
    fn convert(&self) -> Self::Output {
        42
    }
}

fn main() {
    let my_int = MyInt;

    let output = my_int.convert();
    println!("output is: {}", output);


}
