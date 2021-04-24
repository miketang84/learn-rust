

fn f1_len(s: &String) -> usize {
    println!("in f1_len: {}", s);
    s.len()
}

fn f2_len(s: &mut String) -> usize {
    println!("in f2_len: {}", s);
    s.len()
}

fn f3_len(s: String) -> usize {
    println!("in f3_len: {}", s);
    s.len()
}


fn main() {
    let mut astr = String::from("abcde");
    let r = f1_len(&astr);
    println!("astr len is: {}", r);
    println!("astr is: {}", astr);
    let r = f2_len(&mut astr);
    println!("astr len is: {}", r);
    println!("astr is: {}", astr);
    let r = f3_len(astr);
    println!("astr len is: {}", r);
    // try to uncomment this line, to see compiler complaint info
    // println!("astr is: {}", astr);

    println!("Hello, world!");
}
