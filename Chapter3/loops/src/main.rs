fn main() {
    let mut stop_for: i32 = 0;

    // infinite loop
    loop {
        println!("for looping");

        stop_for += 1;
        if stop_for == 100 {
            break;
        }
    }

    let mut stop_while: i32 = 0;
    while stop_while < 100 {
        println!("while looping");
        stop_while += 1;
    }

    let numbers = [7, 13, 17, 23, 66];
    let mut idx = 0;

    // iterate over the array with a while. unsafe (can panic)
    while idx < 5 {
        println!("idx {}, value {}", idx, numbers[idx]);
        idx += 1;
    }

    // better aproach. use reference
    for (idx, num) in numbers.iter().enumerate() {
        println!("idx {}, value {}", idx, num);
    }

    // this one copy the elements. (slower with big ds)
    for &num in &numbers {
        println!("idx {}, value {}", idx, num);
    }

    for n in (0..10).rev() {
        println!("{}!", n);
    }
    println!("LIFTOFF!!!");
}
