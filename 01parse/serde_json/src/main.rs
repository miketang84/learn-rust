use serde::Deserialize;
use serde_json::value::Value;

#[derive(Deserialize, Debug)]
struct User {
    fingerprint: String,
    location: String,
}

fn main() {
    // The type of `j` is `&str`
    let j = "
        {
            \"fingerprint\": \"0xF9BA143B95FF6D82\",
            \"location\": \"Menlo Park, CA\"
        }";

    let u: User = serde_json::from_str(j).unwrap();
    println!("{:#?}", u);

    let u: Value = j.parse().unwrap();
    println!("{:#?}", u);

    //let u: User = j.parse().unwrap();
    //println!("{:#?}", u);

    // error[E0277]: the trait bound `User: std::str::FromStr` is not satisfied
    //   --> src/main.rs:24:21
    //    |
    // 24 |     let u: User = j.parse().unwrap();
    //    |                     ^^^^^ the trait `std::str::FromStr` is not implemented for `User`


}
