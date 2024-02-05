use std::{error::Error, io::{stdin, Read}};
use std::fmt;

enum Grade {
    A,
    B,
    C,
    D,
    F,
}

impl Grade {
    fn from_score(score: f32) -> Grade {
        let score_int = score as i32;
        match score_int {
            90..=100 => Grade::A,
            80..=89 => Grade::B,
            70..=79 => Grade::C,
            60..=69 => Grade::D,
            _ => Grade::F,
        }
    }
}

impl fmt::Display for Grade {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Grade::A => write!(f, "You got an A!"),
            Grade::B => write!(f, "You got a B!"),
            Grade::C => write!(f, "You got a C!"),
            Grade::D => write!(f, "You got a D!"),
            Grade::F => write!(f, "You got an F!"),
        }
    }
}

struct Notes {
    first_vize: f32,
    second_vize: f32,
    final_exam: f32,
    first_vize_percentage: f32,
    second_vize_percentage: f32,
    final_exam_percentage: f32,
}

impl Notes {
    fn new() -> Notes {
        Notes {
            first_vize: 0.0,
            second_vize: 0.0,
            final_exam: 0.0,
            first_vize_percentage: 0.0,
            second_vize_percentage: 0.0,
            final_exam_percentage: 0.0,
        }
    }

    fn calculate(&self) -> f32 {
        let result = (self.first_vize * (self.first_vize_percentage)*0.01)
         + (self.second_vize * (self.second_vize_percentage)*0.01)
          + (self.final_exam * (self.final_exam_percentage)*0.01);
        result
    }

    fn take_note(&mut self, note_name: &str) -> Result<(), Box<dyn Error>> {
        println!("Enter the {} note", note_name);
        let mut input = String::new();
        stdin().read_line(&mut input)?;
        let note: f32 = input.trim().parse()?;
        match note_name {
            "first vize" => self.first_vize = note,
            "second vize" => self.second_vize = note,
            "final exam" => self.final_exam = note,
            _ => (),
        }
        Ok(())
    }
    
    fn take_percentage(&mut self, percentage_name: &str) -> Result<(), Box<dyn Error>> {
        println!("Enter the {} percentage", percentage_name);
        let mut input = String::new();
        stdin().read_line(&mut input)?;
        let percentage: f32 = input.trim().parse()?;
        match percentage_name {
            "first vize" => self.first_vize_percentage = percentage,
            "second vize" => self.second_vize_percentage = percentage,
            "final exam" => self.final_exam_percentage = percentage,
            _ => (),
        }
        Ok(())
    }
}

fn main() {
    let mut notes = Notes::new();

   
    notes.take_note("first vize").unwrap();
    notes.take_percentage("first vize").unwrap();

    notes.take_note("second vize").unwrap();
    notes.take_percentage("second vize").unwrap();

    notes.take_note("final exam").unwrap();
    notes.take_percentage("final exam").unwrap();

    let result = notes.calculate();
    let grade = Grade::from_score(result);
    println!("{}", grade);
}