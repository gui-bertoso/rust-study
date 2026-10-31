use std::io;

fn main() {
    loop {
        println!("Welcome to area calculator!");

        print!("Type the Height >> ");
        let mut input1 = String::new();
        io::stdin().read_line(&mut input1).expect("Failed to read line");

        print!("Type the Width >> ");
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

        let area = (a, b);

        let result = calculate_area(area);
        println!("The area of {a}x{b} are {result}")
    }
}


fn calculate_area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
