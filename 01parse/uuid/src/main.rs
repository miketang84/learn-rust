use uuid::Uuid;

fn main() {
    let astr = "97103fab-1e50-36b7-0c03-0938362b0809";
    
    let auuid = Uuid::parse_str(astr).unwrap();
    println!("{:?}", auuid);

    let auuid: Uuid = astr.parse().unwrap();
    println!("{:?}", auuid);
}
