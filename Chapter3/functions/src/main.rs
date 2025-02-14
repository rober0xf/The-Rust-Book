fn main() {
    println!("Hello, world!");

    println!("return {}", another_function(100));
}

fn another_function(x: i64) -> i64 {
    10 + x
}
