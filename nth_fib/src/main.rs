use std::io;

fn main() {
    println!("Enter which Fib sequence value you want: ");

    let input = obtain_user_input();

    let solution = get_fib_number( input );

    println!("answer is {}", solution)
}

fn obtain_user_input() -> u32 {
    let mut sequence = String::new();

    io::stdin()
        .read_line(&mut sequence)
        .expect("Failed to read line");

    let sequence: u32 = sequence.trim().parse().expect("Please enter a number");

    sequence
}

fn get_fib_number( input: u32 ) -> u32 {
    //1,1,2,3,5,8
    if input == 1 || input == 2 {
        return 1
    };

    get_fib_number( input - 1 ) + get_fib_number( input - 2 )
}
