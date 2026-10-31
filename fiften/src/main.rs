struct Color(i32, i32, i32);

#[derive(Debug)]
struct Vector2D(i32, i32);

struct AlwaysVisible; // what?

fn main() {
    println!("Hello, world!");

    let black = Color(0, 0, 0);
    let position = Vector2D(0, 0);

    let is_visible = AlwaysVisible; //what ??????

    println!("Your position are {position:?}"); // :? and derive(Debug) permits print ok, deriving of std:fmt::Display from Debug

    dbg!(position); // print debug of a struct

}
