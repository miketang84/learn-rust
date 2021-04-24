use std::io::{Read, Write};

mod student;

use student::{Student, Gender, Output1};

fn main() -> std::io::Result<()> {
    let mut file = std::fs::File::open("students_in.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("{}", contents);

    let lines: Vec<&str> = contents.trim().split('\n').collect();
    println!("{:?}", lines);


    let mut students: Vec<Student> = Vec::new();

    for line in lines {
        let objv: Vec<&str> = line.split(",").collect();
        println!("{:?}", objv);
        let name = objv[0].to_string();
        let gender = if objv[1] == "female" {
            Gender::Female
        }
        else {
            Gender::Male
        };
        let age: usize = objv[2].parse().unwrap();
        let score: usize = objv[3].parse().unwrap();
        let credit = objv[4].parse::<f32>().unwrap();
        let student = Student::new(name, gender, age, score, credit);

        students.push(student);
    }
    println!("{:?}", students);

    let mut output = String::new();
    for mut student in students {
        student.add_score(5);    
        output.push_str(&student.output1());
    }

    let mut f = std::fs::File::create("students_out.txt")?;
    f.write_all(output.as_bytes())?;
    f.sync_all()?;
    
    Ok(())
}


