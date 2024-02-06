// guessing game which takes user input to guess a number and outputs if use got it right or wrong

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number (1-100)!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // generates a random number between 1 and 100

    //println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");
        
        let mut guess = String::new(); // mut is used to make a variable mutable which means that the value can be changed compared to immutable which is the opposite
        
        io::stdin()
            .read_line(&mut guess) // .read_line to receive user input/read it, &mut guess is a reference to the variable guess (stores input in var guess)
            .expect("Failed to read line"); // .expect is used to handle errors, if .read_line returns an error, .expect will print the message passed to it and crash the program
        
        //let guess: u32 = guess.trim().parse().expect("Please type a number!");
            // creates guess which shadows the original guess var
            // trim() is used to remove any whitespace from the input
            // parse() parses the string into a number, here we use it to convert the string into an unsigned 32-bit integer (hence u32)
        
        let guess: u32 = match guess.trim().parse() { 
            // compared with the above code block used to convert string into number then crashes if not a number, 
            // this ignores any non number guesses and allows the user to continue guessing
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small! Guess higher!"),
            Ordering::Greater => println!("Too big! Guess lower!."),
            Ordering::Equal => {
                println!("You win!");
                break;
                // breaks out of loop if correct guess reached
            }
        }
    }
}