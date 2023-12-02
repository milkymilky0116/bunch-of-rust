use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number!");

    let answer = rand::thread_rng().gen_range(1, 101);
    let mut trial = 10;
    println!("Randomly generated number is : {}", answer);
    loop {
        println!("Please input your guess");
        let mut guess = String::new();
        match io::stdin().read_line(&mut guess) {
            Ok(_) => println!("the number that you entered is : {}", guess),
            Err(_) => println!("Please Enter proper input."),
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&answer) {
            Ordering::Greater => {
                println!("your number is greater than answer");
                trial -= 1;
                println!("Remain trial : {}", trial);
            }
            Ordering::Less => {
                println!("your number is less than answer");
                trial -= 1;
                println!("Remain trial : {}", trial);
            }
            Ordering::Equal => {
                println!("Correct!");
                break;
            }
        }

        match trial {
            0 => {
                println!("You're out of trial! Actual answer : {}", answer);
                break;
            }
            _ => continue,
        }
    }
}
