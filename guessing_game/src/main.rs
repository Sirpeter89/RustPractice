use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new(); //mut makes it mutable, by default variables are immutable

    io::stdin() //calling stdin function from io
    //can also be std::io::stdin without import
        .read_line(&mut guess)//need to make reference to guess mutable.
        //.read_line returns io::Result
        .expect("Failed to read line");
        //io::Result has expect method we're calling
        //if it returns an error, it will output string, else returns Ok value
        //which is users input.

    println!("You guessed: {}", guess);
}
