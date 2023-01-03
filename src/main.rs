use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    println!("Adivinhe um numero!");

    let secret_number = rand::thread_rng().gen_range(1..=10);

    loop {
        println!("Digite seu palpite (1 a 10).");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Seu palpite: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Seu numero é menor!"),

            Ordering::Greater => println!("Seu numero é maior!"),

            Ordering::Equal => {
                println!("Parabéns, voce venceu!");
                break;
            }
        };
    }
}
