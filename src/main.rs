use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    
    //println!("The secret number is: {secret_number}");
    loop {
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {}", guess);
    let joke_number = rand::thread_rng().gen_range(1..=5);
    match guess.cmp(&secret_number) {
        Ordering::Less => {
            println!("Too small!");
            match joke_number{
                1=> println!("Even Gadhaprasad could guess!"),
                2=> println!("You are not pretty enough to be this stupid."),
                3=> println!("I envy people who have not met you."),
                4=> println!("You will be utterly forgotten."),
                5=> println!("Two wrongs don’t make a right, take your parents as an example."),

                _=> println!("No puns!"),

            } 
        }
        Ordering::Greater => {println!("Too big!");
            match joke_number{
                1=> println!("Even Gadhaprasad could guess!"),
                2=> println!("You are not pretty enough to be this stupid."),
                3=> println!("I envy people who have not met you."),
                4=> println!("You will be utterly forgotten."),
                5=> println!("Two wrongs don’t make a right, take your parents as an example."),
                
                _=> println!("No puns!"),
            }
        }
        Ordering::Equal => {
            println!("You win!");
            break;
        }
        }
    }
}