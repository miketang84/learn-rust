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



impl Student {
    fn output(&self) {
        let mut out = String::new();
        out = out + &self.name + "," + &self.gender.text() + "," + &self.age.to_string();

        println!("{}", out);
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

    lily.output();
    bob.output();
}


