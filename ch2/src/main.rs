use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng()
        .gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        /*
        * Note:
        *
        * In Rust, variables are immutable by default!
        *
        * Also, :: is an indication of associated functions,
        * a.k.a. static methods.
        */
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        guess = guess
            .trim()
            .to_string();

        let guess: u32 = match guess
            .parse() {
                Ok(num) => num,
                Err(e) => {
                    println!("Wrong input: {}.", guess);
                    println!("Reason: {}.", e);
                    println!("Please input another one!");
                    continue;
                },
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
