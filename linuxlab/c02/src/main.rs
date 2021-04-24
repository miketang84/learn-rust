

fn str_len(s: String) -> usize {
    println!("in str_len: {}", s);
    s.len()
}

fn main() {
    let astr = String::from("abcde");
    let r = str_len(astr);
    println!("astr len is: {}", r);
}
