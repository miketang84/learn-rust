mod student;

use student::{Student, Gender, Output1, Output2};

fn main() {
    let lily = Student::new(
        String::from("lily"),
        Gender::Female,
        18,
        88,
        4.2);

    let bob = Student::new(
        String::from("bob"),
        Gender::Male,
        18,
        92,
        4.5);

    lily.output1();
    lily.output2();
    bob.output1();
    bob.output2();
}


