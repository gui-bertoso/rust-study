fn main() {
    println!("Go drinking some shots of tequila");

    for number in (1..5).rev() {
        let condition = number == 5;

        if condition { println!("{number}!") } else { println!("{number}, ") };
    }
    println!("DRUNKK")
}
