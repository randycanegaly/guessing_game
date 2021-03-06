use std::io;
use std::cmp::Ordering;
use rand::Rng;

//Randy's final version
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1,101);
    //println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line.");

            if guess.trim() == "quit" {
                //println!("I see quit!");
                break;
            } else {
                println!("guess is {}", guess);
            }

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                //Err("quit") => break,
                Err(_) => continue
            };

            println!("You guessed: {}", guess);

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("You win!");
                    break;
                }
        }
    }
}
