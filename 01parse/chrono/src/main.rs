use chrono::NaiveDateTime;

fn main() {
    let astr = "2015-09-05 23:56:04";

    let ndt = NaiveDateTime::parse_from_str(astr, "%Y-%m-%d %H:%M:%S").unwrap();
    println!("{:?}", ndt);

    // This will cause error, for the reason of unknown format
    // let ndt: NaiveDateTime = astr.parse().unwrap();
    // println!("{:?}", ndt);
    
    // Let's give the default format.
    let astr = "2015-09-18T23:56:04";
    let ndt: NaiveDateTime = astr.parse().unwrap();
    println!("{:?}", ndt);

    let astr = "2015-09-18T23:56:04.07";
    let ndt: NaiveDateTime = astr.parse().unwrap();
    println!("{:?}", ndt);

}
