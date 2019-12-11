fn main() {

    let astr = "7";
    let n = astr.parse::<i32>().unwrap();
    println!("{:?}", n);
    let n = astr.parse::<i64>().unwrap();
    println!("{:?}", n);
    let n = astr.parse::<u32>().unwrap();
    println!("{:?}", n);
    let n = astr.parse::<u64>().unwrap();
    println!("{:?}", n);

    let astr = "7.42";
    let n: f32 = astr.parse().unwrap();
    println!("{:?}", n);
    let n: f64 = astr.parse().unwrap();
    println!("{:?}", n);

}
