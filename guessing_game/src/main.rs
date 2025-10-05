use std::io; // std library input/output

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new(); //las variables son inmutables agregar mut para hacerla mutable

    io::stdin()
        .read_line(&mut guess)  // & indica que es una referencia es inmutable, &mut es muta
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}