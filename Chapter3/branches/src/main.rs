fn main() {
    let age = 19;

    if age > 18 {
        println!("you can get in");
    } else {
        println!("too young");
    }

    let condition = true;
    let _number = if condition { 666 } else { 420 };
    // DIFFERENT TYPE: let _diff = if condition { 666 } else { "fiz" };

}
