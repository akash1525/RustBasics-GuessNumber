use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    let number = rand::thread_rng().gen_range(1,11);
    println!("number: {}", number);

    loop {
        println!("Guess a number between 1 and 10:");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);

    let guess: u32 = guess.trim().parse().expect("Please type a number");
    println!("guess as a number: {}", guess);

    match guess.cmp(&number) {
        Ordering::Less => println!("Too Low"),
        Ordering::Greater => println!("Too High"),
        Ordering::Equal => {
            println!("You Win");
            break;
        }
    }
    }
}
