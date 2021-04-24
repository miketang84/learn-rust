#[derive(Debug)]
pub enum Gender {
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
pub struct Student {
    name: String,
    gender: Gender,
    age: usize,
    score: usize,
    credit: f32,
}

impl Student {
    pub fn new(name: String, gender: Gender, age: usize, score: usize, credit: f32) -> Student {
        Student {
            name,
            gender,
            age,
            score,
            credit
        }
    }

    pub fn add_score(&mut self, delta: usize) {
        self.score += delta;
    }
}


pub trait Output1 {
    fn output1(&self) -> String;
}

pub trait Output2 {
    fn output2(&self) {
        let out = String::from("call general output2");

        println!("{}", out);
    }
}

impl Output1 for Student {
    fn output1(&self) -> String {
        let out = format!("{}, {}, {}, {}, {:.2}\n", self.name, self.gender.text(), self.age, self.score, self.credit);
        out
    }
}

impl Output2 for Student {}
