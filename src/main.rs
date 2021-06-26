mod exercises;
use std::string::*;


fn main() {
    println!("Hello, Rust!");
    println!();
    let stdin = std::io::stdin();
    let mut str = String::new();
    loop {
        str.clear();
        println!("======================= enter day num to show your work! =====================================");
        println!();
        stdin.read_line(&mut str).unwrap();
        let day = str.trim().parse::<i32>().unwrap();
        if day == 0 {
            println!("bye bye");
            break;
        }
        println!("here is your no.{} day work.", &str.trim());
        exercises::exec(day);
        println!();
        println!("======================= that's over all your day work. =======================================");
        println!();
        println!();
    }
}