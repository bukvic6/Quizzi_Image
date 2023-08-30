use serde::Deserialize;
use serde_json::Result as SerdeResoult;
use image::{ImageResult};
use std::fs;
use std::io;

#[derive(Deserialize, Debug)]
struct Data {
    question: String,
    answer: Vec<Answer>,
}

#[derive(Debug, Deserialize)]
struct Answer {
    number: f32,
    text: String,
}

fn main() -> SerdeResoult<()> {
    let data = fs::read_to_string("./src/quiz.json").expect("Unable to read file");
    let quiz_data: Vec<Data> = serde_json::from_str(&data).expect("JSON was not well-formatted");
    const WIDTH: usize = 3;
    const HEIGHT: usize = 3;
    println!("\n\nAre you ready to discover your ultimate level of");
    println!("whimsy and receive your one-of-a-kind picture? Let's get started!");
    println!("\n---------------------------------------------------------------\n");
    println!("\nRemember, you're arranging the numbers from most to least likely\n");


    let mut new_one_dim: Vec<u8> = Vec::with_capacity(3 * WIDTH * HEIGHT);
    for question in quiz_data.iter() {
        println!("{:?}", question.question);
        for ans in question.answer.iter() {
            println!("{:?}", ans.text);
        }
        loop {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let input_values: Result<Vec<u32>, _> = input
                .split_whitespace()
                .map(|x| x.trim().parse::<u32>())
                .collect();

            match input_values {
                Ok(parsed_values) => {
                    let valid_values: [u32; 3] = [1, 2, 3];
                    if !valid_values
                        .iter()
                        .all(|value| parsed_values.contains(value))
                    {
                        println!("must enter numbers from 1 to 3");
                        continue;
                    }
                    println!("Choose number from 0 to 850");
                    let mut random_number = String::new();
                    io::stdin()
                        .read_line(&mut random_number)
                        .expect("Failed to read line");
                    let random_number: u32 = match random_number.trim().parse() {
                        Ok(num) => num,
                        Err(_) => continue,
                    };
                    let n = 3;
                    for i in 0..n {
                        let result_f32 = question.answer[i].number * random_number as f32;
                        let result_u8 = result_f32 as u8;

                        new_one_dim.push(result_u8);
                    }
                    break;
                }
                Err(e) => {
                    eprintln!("Error parsing input: {}", e);
                    continue;
                }
            }
        }
    }
    let result: ImageResult<()> = image::save_buffer("quizzi_image.png", &new_one_dim, WIDTH as u32, HEIGHT as u32, image::ColorType::Rgb8);
    match result {
        Ok(_) => println!("Your image is created"),
        Err(err) => eprintln!("Error saving image: {}",err),
        
    }

    Ok(())
}
