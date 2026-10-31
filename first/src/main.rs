use std::io; // std -> standart library // io -> input output lib // use -> import lib

fn main() { // initial funcion as if __name__ == "__main__"

    println!("Hello, world!");

    println!("Input a number \n  >>");

    let mut input = String::new(); // String::new() create a empty instance of String Object

    io::stdin().read_line(&mut input).expect("failed to read line");

    println!("Your guess -> {input}")
}
