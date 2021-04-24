#[derive(Debug)]
enum Gender {
    Female,
    Male,
}

impl Gender {
    fn text(&self) -> String {
        match *self {
            Gender::Female => {
                "female".to_string()
            },
            Gender::Male => {
                "male".to_string()
            }
        }
    }
}


#[derive(Debug)]
struct Student {
    name: String,
    gender: Gender,
    age: usize,
    score: usize,
    credit: f32,
}


trait Output1 {
    fn output1(&self);
}

trait Output2 {
    fn output2(&self) {
        let out = String::from("call general output2");

        println!("{}", out);
    }
}

impl Output1 for Student {
    fn output1(&self) {
        let mut out = String::new();
        out = out + &self.name + "," + &self.gender.text() + "," + &self.age.to_string();

        println!("{}", out);
    }
}

impl Output2 for Student {}

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

    lily.output1();
    lily.output2();
    bob.output1();
    bob.output2();
}


