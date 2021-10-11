use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess:String = String::new();
    /*
        1 ) argument type is similar to local variable guess but is reference type
        2 ) the argument for expect is string literal
     */

    io::stdin().read_line(&mut guess).expect("Failed to read line");
    println!("You guessed: {}", guess);


}
