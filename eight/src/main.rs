use rand::prelude::*;

fn main() {
    println!("------------------------------------");
    println!("     can i get out home today?      ");
    println!("------------------------------------");

    let rand_number = rand::rng().random_range(1..=10);
    let another_rand_number = rand::rng().random_range(1..=5);

    if rand_number < 3 {
        let condition = another_rand_number > 2;
        let out_text = if condition { "You dont have lucky, dont get out your home today" } else { "Dont get out your home today, for your security" };

        println!("{out_text}");
    } else {

        let condition = another_rand_number < 2;
        let out_text = if condition { "You have so many lucky, can get out your home :)" } else { "Wow, are you lucky, can get  out your home, have a nice day :)" };

        println!("{out_text}");
    }
}
