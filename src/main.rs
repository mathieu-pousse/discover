extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Hello, world!");

    let to_guess: i32 = rand::thread_rng().gen_range(1, 101);
    println!("{}", to_guess);
    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).expect("bitch");
        println!("sent: {}", line);
        let candidate: i32 = match line.trim().parse() {
            Ok(number) => number,
            Err(error) => {
                println!("illegal value {}", error);
                continue;
            }
        };
        match to_guess.cmp(&candidate) {
            Ordering::Less => println!("Less"),
            Ordering::Greater => println!("Greater"),
            Ordering::Equal => {
                println!("found {}", to_guess);
                break;
            } 
        }
    }

}
