enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    let a = value_in_cents(Coin::Nickel);
    println!("Hello, your currency are: {a}!");
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
        _ => {
            println!("Invalid coin");
            0
        },
    }
}


// notes
//
// packages -> a cargo feature thath lets you build, text and share crates
// crates -> A tree of modules that produces a library or executable
// modules and use -> let you control the organization, scope, and privacy of paths
// paths -> a way of naming an item, such as a struct, function, or module


// cheat sheet
//
// to use other files
//
// pub mod vegetables;
//
// pub struct Aspargus {}
//
// pub -> public
//
// crate::garden::vegetables::Aspargus
    //backyard
    //├── Cargo.lock
    //├── Cargo.toml
    //└── src
    //    ├── garden
    //    │   └── vegetables.rs
    //    ├── garden.rs
    //    └── main.rs
