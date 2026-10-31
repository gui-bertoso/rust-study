use std::io;

fn main() {
    loop {
        println!("Welcome to calculator (multiplication only)!");

        print!("Type the first number >> ");
        let mut input1 = String::new();
        io::stdin().read_line(&mut input1).expect("Failed to read line");

        print!("Type the second number >> ");
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("Failed to read line");

        println!("");

        let a: u32 = match input1.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let b: u32 = match input2.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let result = multiply(a, b);
        println!("{a} * {b} = {result}")
    }
}


fn multiply(a: u32, b: u32) -> u32 {
    a * b
}
