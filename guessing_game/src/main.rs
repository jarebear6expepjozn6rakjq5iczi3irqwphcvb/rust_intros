use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("🔥 PICK THE RIGHT NUMBER maybe 🔥");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");

    loop {
        println!("Input your guess, maybe");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too smol"),
            Ordering::Greater => println!("too large"),
            Ordering::Equal => {
                println!("winner chicken dinn dinn");
                break;
            },
        }
    }
}
