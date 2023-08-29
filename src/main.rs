use serde_json::Result;
use std::fs;
use serde::{Deserialize};


#[derive(Deserialize)]
#[derive(Debug)]
struct Data {
    question: String,
    answer: Vec<Answer>

}
#[derive(Debug)]
#[derive(Deserialize)]
struct Answer {
    number: String,
    text: String,
}
fn main() -> Result<()>{
    let data = fs::read_to_string("./src/quiz.json").expect("Unable to read file");
    let quiz_data: Vec<Data> = serde_json::from_str(&data).expect("JSON was not well-formatted");

    for question in quiz_data.iter(){
        println!("{:?}", question.question);
        for ans in question.answer.iter(){
            println!("{:?} number: {:?}",ans.text, ans.number)
        }
    } 

    Ok(())
}
