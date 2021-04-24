#[derive(Debug)]
enum Gender {
    Female,
    Male,
}

#[derive(Debug)]
struct Student {
    name: String,
    gender: Gender,
    age: usize,
    score: usize,
    credit: f32,
}

fn check(stu: Student) {

    match stu.gender {
        Gender::Female => {
            println!("It is a girl.")

        },
        Gender::Male => {
            println!("It is a boy.")
        }
    }
}

fn main() {
    let lily = Student {
        name: String::from("lily"),
        gender: Gender::Female,
        age: 18,
        score: 88,
        credit: 0.0
    };

    let bob = Student {
        name: String::from("bob"),
        gender: Gender::Male,
        age: 18,
        score: 92,
        credit: 0.0
    };

    check(lily);
    check(bob);
}
