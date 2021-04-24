#[derive(Debug)]
struct Student {
    name: String,
    gender: u8,
    age: usize,
    score: usize,
    credit: f32,
}


fn main() {
    let lily = Student {
        name: String::from("lily"),
        gender: 0, // 0 is female
        age: 18,
        score: 88,
        credit: 0.0
    };
    println!("The student info is: {:?}", lily);
    println!("The student info is: {:#?}", lily);
}
