use colorful::{Color, Colorful};
use rand::thread_rng;
use rand::Rng;
use std::io::{Error, Read};
use std::num::ParseIntError;

pub fn guess_number() {
    println!("{}", "start guess number...".blue());
    let rand_num = rand::thread_rng().gen_range(1..100);
    let mut input = String::new();
    let mut input_max = 100;
    let mut input_min = 0;
    loop {
        println!("please input your number.");
        std::io::stdin().read_line(&mut input).unwrap();
        let result = input.trim().parse::<i32>();
        match result {
            Ok(num) => {
                if num == rand_num {
                    println!("{}", "congratulations!".green());
                    break;
                }
                if num < rand_num {
                    println!("your answer is small!");
                    input_min = num;
                } else {
                    println!("your answer is too big");
                    input_max = num;
                }
                println!(
                    "{} tips: {} - {}",
                    "come on!".gradient(Color::Green),
                    &input_min,
                    &input_max
                );
            }
            Err(_) => {
                println!("{}", "please input a valid number!".red());
            }
        }
        input.clear();
    }
}
