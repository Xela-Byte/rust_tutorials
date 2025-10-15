use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Input your guess: ");
        let mut input_number: String = String::new();

        io::stdin()
            .read_line(&mut input_number)
            .expect("Failed to read line");

        let guess: u32 = match input_number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small..."),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Greater => println!("Too big..."),
        }
    }
}
