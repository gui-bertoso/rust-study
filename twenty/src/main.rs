enum Message {
    Quit,
    Move {x:i32, y:i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum Option<T> {
    None,
    Some(T),
}

impl Message {
    fn call(&self) {
        println!("Message here")
    }
}

fn main() {
    let m = Message::Write(String::from("Hello"));
    m.call();

    let some_number = Some(5);
    let some_char = Some('e');

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y;
}
