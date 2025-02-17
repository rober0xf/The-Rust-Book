use std::io;
fn main() {
    println!("do you want to convert celsius to fahrenheit? press 1");
    println!("do you want to convert fahrenheit to celsius? press 2");

    let option = read_number();

    match option {
        1 => {
            println!("celsius to convert: ");
            let amount = read_float();
            let result = celsius_to_fahrenheit(amount);
            println!("{:.2} celsius are {:.2} fahrenheit", amount, result);
        }
        2 => {
            println!("fahrenheit to convert: ");
            let amount = read_float();
            let result = fahrenheit_to_celsius(amount);
            println!("{:.2} fahrenheit are {:.2} celsius", amount, result);
        }
        _ => println!("invalid input"),
    }
}

fn read_number() -> i8 {
    loop {
        let mut input_line = String::new();
        io::stdin()
            .read_line(&mut input_line)
            .expect("error reading input");
        match input_line.trim().parse::<i8>() {
            Ok(num) => return num,
            Err(_) => println!("invalid input"),
        }
    }
}

fn read_float() -> f64 {
    loop {
        let mut input_line = String::new();
        io::stdin()
            .read_line(&mut input_line)
            .expect("error loading input");
        match input_line.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("invalid input"),
        }
    }
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}
