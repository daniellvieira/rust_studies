// Para gerar a documentação: cargo doc --open

// The io library comes from the standard library, known as std
use std::io;
use std::cmp::Ordering;
use rand::Rng;

// By default, Rust has a set of items defined in the standard library
// https://doc.rust-lang.org/stable/std/prelude/index.html
// https://crates.io/

fn main() {
  println!("Guess the number!");

  let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

  // println!("The secret number is: {secret_number}");

  loop {
    println!("Please input your guess.");

    // we’ll create a variable to store the user input
    let mut guess: String = String::new();

    // The & indicates that this argument is a reference
    // variables are immutable by default
    // If you don’t call expect, the program will compile, but you’ll get a warning
    io::stdin()
      .read_line(&mut guess)
      .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };

    println!("You guessed: {guess}");

    // Rust has a strong, static type system
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
