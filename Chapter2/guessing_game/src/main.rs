extern crate rand; // use the rand crate

use rand::Rng; // the Rng defines methods that random number generators implement
use std::cmp::Ordering; // for compare the two values. returns: less, equal or greater
use std::io; // for inputs

fn main() {
    println!("welcome to the guessing game!");
    println!("input your number: ");

    // the rand::rng will give us the random number generator that we are going to use,
    // one that is local to the current thread of execution and seeded by the OS.
    // next we call random_range. this method takes two numbers and generates a random number.
    let secret_number = rand::rng().random_range(1..101);

    loop {
        // this defines an empty mutable string. by default the variables are immutables
        let mut guess = String::new();

        // reads the input from the user and store it in guess. references are immutables by default,
        // so we need to pass the &mut. and the expect method returns an Ok if the operation didnt fail
        // and an Err if its fail and will return that message
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        // cast the string to uint32. the trim method will eliminate any whitespace at the beginning
        // and at the end. the parse method parses the string to u32 in this case
        // switching from an expect expression to a match expression is how we move from crashing
        // to handling the error.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // if its not a number ignore and go to the 2nd loop
        };

        // the cmp method compares the two values, and it takes the reference to what you want to compare with
        // the match will run which pattern is correct
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("too high!"),
            Ordering::Equal => {
                println!("nailed it!");
                break;
            }
        }

        // println!("your guess: {}", guess);  we use the placeholder with {}
    }
}
