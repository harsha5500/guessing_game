use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let welcome_message = "Welcome to the guessing game";
    let input_message = "Please input your guess.";

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("{}", welcome_message);
    println!("{}", input_message);

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read a line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
