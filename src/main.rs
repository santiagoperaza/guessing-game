use std::io;
use std::cmp::Ordering;
use rand::Rng;

const TITLE: &str = r"+-------------------------------------------------------+
|          Guess the number between 1 and 100!          |
|                                                       |
|          Enter 'hint' if you need some help!          |
+-------------------------------------------------------+";

fn main() {
    println!("\n{TITLE}\n");
    println!("Enter 'hint' if you need some help!");

    const HINT: &str = "hint";
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut attempts: u32 = 0;
    
    loop { 
        println!("Please input your guess.");
        attempts += 1;
        let mut guess = String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        if guess.trim() == HINT {
            display_hint(secret_number);
        } else {       
            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("❌ That's not a number!");
                    continue;
                },
            };
            
            println!("You entered: {guess}");
            
            match guess.cmp(&secret_number) {
                Ordering::Less => println!("❌ Too small!"),
                Ordering::Greater => println!("❌ Too big!"),
                Ordering::Equal => {
                    println!("You win! 🏆, number of attempts: {}", attempts);
                    break;
                }
            }
        }
    }
}

fn display_hint(secret_number: u32) {
    let hint_message = if secret_number % 2 == 0 { "The secret number is even!" } else { "The secret number is odd!" };
    println!(r"+-------------------------------------------------------+
|               {hint_message}               |
+-------------------------------------------------------+");
}