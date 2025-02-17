use std::io;
fn main() {
    println!("gimme a number and i'll give you its fibonacci number");

    let input = read_number();

    // cast to usize
    let input_size = input as usize;

    let result = fib(input_size);
    println!("the ith fib of {} is: {}", input, result);
}

fn read_number() -> i64 {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("error reading input");
        match input.trim().parse::<i64>() {
            Ok(num) if num >= 0 => return num,
            Ok(_) => println!("give me a valid input"),
            Err(_) => println!("invalid input"),
        }
    }
}

fn fib(ith: usize) -> i64 {
    if ith == 0 {
        return 0;
    }

    /* this macro allows us to create a vector with initialized values. in this case of size ith + 1 and every element is a 0 */
    let mut dp: Vec<i64> = vec![0; ith + 1];

    dp[1] = 1;

    // we skip the first two
    for n in 2..=ith {
        dp[n] = dp[n - 1] + dp[n - 2];
    }

    dp[ith]
}
