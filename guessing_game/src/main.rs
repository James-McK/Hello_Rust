use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // Pick a random number between 1 and 100 as the secret number
    let secret_number = rand::thread_rng().gen_range(1..101);
    // User a Vector to keep track of all the guesses so far
    let mut guesses: Vec<i32> = Vec::new();

    // Print out the game's title
    println!(
        "\
**********************
 James' Guessing Game
**********************"
    );

    // Keep looping until the user guesses correctly
    loop {
        // Ask the user for their input
        println!("Please input your guess: ");

        // Read in the input from stdio
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        // Try to parse the user's input into an integer
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                // If it can't be parsed, let the user know they entered something invalid
                println!("Please enter a valid number!");
                // And begin asking them for input again
                continue;
            }
        };

        // Let the user know if they try to make a guess outside the valid range of numbers
        if guess < 0 || guess > 100 {
            println!("Please enter a number in the correct range (0-100)!");
            continue;
        }

        // Add the guess to the list of guesses
        guesses.push(guess);
        // And print out all the guesses so far
        print!("\nGuesses so far: ");
        for i in 0..(guesses.len() - 1) {
            print!("{}, ", guesses[i]);
        }
        println!("{}.", guesses[guesses.len() - 1]);

        // Compare the user's guess to the secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Low!"),
            Ordering::Greater => println!("Too High!"),
            Ordering::Equal => {
                // If they guessed correctly, let them know and exit the game
                println!("You Win!");
                break;
            }
        }
    }
}
