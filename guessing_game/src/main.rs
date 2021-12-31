use std::io;
use rand::Rng; //need Rng traits to properly use rand.
use std::cmp::Ordering; //Orderering (like Result) is an enum, 3 variants, less, greater, equal

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    //thread_rng is random num gen that comes from rand
    //.gen_range comes from Rng gives us a range. inclusive lower, exclusive upper
    //could also write 1..=100 equivalent

    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); //mut makes it mutable, by default variables are immutable

        io::stdin() //calling stdin function from io
        //can also be std::io::stdin without import
            .read_line(&mut guess)//need to make reference to guess mutable.
            //.read_line returns io::Result
            .expect("Failed to read line");
            //io::Result has expect method we're calling
            //if it returns an error, it will output string and quit, else returns Ok value
            //which is users input.

        // let guess: u32 = guess.trim().parse().expect("Please type a number");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, //parse put the num value into Ok so Ok(num) would be the return.
            Err(_) => continue, // underscore is catch all value, continue moves us to next iteration
        }; //using match to allow continuation after invalid input
        //"Shadows" guess variable, converts it to int
        //trim gets rid of whitespace and \n chars, pressing enter in io creates \n
        //parse actually converts it to an int, but we have to tell it which type of int
            //here we want u32, unsigned 32 bit int
            //parse returns a Result type, because it can have an error, invalid chars
                //can't convert so we give it an expect for error handling

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) { //cmp returns variant of Ordering enum
            //cmp infers type from guess, guess u32, infers secret_number as u32
            //match expression made up of "arms(patterns)" below are the arms
            //similar to if coniditons, match looks through to see which arm matches and runs that
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }

}
