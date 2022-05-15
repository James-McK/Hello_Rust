use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);
    let mut guesses: Vec<i32> = Vec::new();

    println!(
        "\
**********************
 James' Guessing Game 
**********************"
    );

    loop {
        println!("Please input your guess: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };
        println!("You guessed {}.", guess);

        if guess < 0 || guess > 100 {
            println!("Please enter a number in the correct range (0-100)!");
            continue;
        }

        guesses.push(guess);
        print!("\nGuesses so far: ");
        for i in 0..(guesses.len() - 1) {
            print!("{}, ", guesses[i]);
        }
        println!("{}.", guesses[guesses.len() - 1]);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Low!"),
            Ordering::Greater => println!("Too High!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}
