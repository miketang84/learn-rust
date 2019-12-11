
use std::str::FromStr;
use std::num::ParseIntError;

/// Parse str like this: "<123>" to 123:i32

#[derive(Debug, PartialEq)]
struct MyInt(i32);

impl FromStr for MyInt {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let astr = &s[1..s.len()-1];
        match astr.parse::<i32>() {
            Ok(n) => Ok(MyInt(n)),
            Err(e) => Err(e)
        }
    }

}

fn main() {
    let astr = "<142>";
    let n: MyInt = astr.parse().unwrap();
    println!("{:?}", n);
}


