use std::io;

fn main() {
    println!("Write a phrase");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");

    //len -> get the usize of a string
    let count = guess.len();
    println!("Your phrase has {count} letters")


    //notes of types
    //
    //
    // SIMPLE TYPES
    //
    // i -> signed | u -> unsigned
    //
    // i -> can be only positive
    // u -> can be positive or negative -> can has a signal - or +
    //
    // dependent-architeture -> isize | usize
    //
    // 8-bit                 -> i8    | u8
    // 16-bit                -> i16   | u16
    // 32-bit                -> i32   | u32
    // 64-bit                -> i64   | u64
    // 128-bit               -> i128  | u128
    //
    //
    // example
    //
    // u8 -> can store number from 0 to 2**8 - 1 (255)
    //
    //
    // integer literals
    //
    // decimal       -> 98_222
    // hex           -> 0xff
    // octal         -> 0o77
    // binary        -> 0b1111_0000
    // byte(u8 only) -> b'A'
    //
    //
    //
    //
    //
    // floats
    //
    // f32 and f64
    // the deafult is f64, but f32 have more precision
    //
    //
    //
    // bool -> used to compares lmao
    //
    //
    //
    //
    //
    // char -> can store a single char
    //
    //
    //
    //
    // COMPOUND TYPES
    //
    // tuple -> can store multiple values of multiple types
    //      example :
    //          let tup: (i32, f64, u8) = (500, 6.4, 1)
    //
    // can you get values with let (a, b, c) = tup;
    //
    //
    //
    //
    // array -> can store multiple values of same type
    // arrays has a fixed size
    //
    // example:
    //      let a = [1,2,3,4,5];    or      let a: [i32; 5] = [1, 2, 3, 4, 5]
    //                                             [type; size]
    //
    //      let b = ["hy", "my", "friend", "how", "are", "you"]
    //
    // can you get value -> let first = a[0]
}
