fn main() {
    // tuples
    let my_tuple: (i32, f32, u8) = (2000, 20.0, u8::MIN);
    let two_thousand = my_tuple.0;
    let twenty = my_tuple.1;
    let max_8 = my_tuple.2;

    // arrays are fixed size
    let months = [
        "january",
        "february",
        "march",
        "april",
        "may",
        "june",
        "july",
        "august",
        "september",
        "october",
        "november",
        "december",
    ];
    // we can access elements using indexing
    let m_first = months[0];
    println!("first month: {}", m_first);

    println!("first element: {}", two_thousand);
    println!("second element: {}", twenty);
    println!("third element: {}", max_8);

    for &month in &months {
        println!("current month: {}", month)
    }
}
