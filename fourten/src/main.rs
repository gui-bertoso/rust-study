struct People {
    name: String,
    age: i32
}

fn main() {
    println!("Welcome, to peoples register !");
    println!("Reading peoples in register");

    let mut people_1 = People {name: String::from("Robert"), age: 32};
    let people_2 = People {
        name: String::from("Alex"),
        age: 27
    };

    people_1.age = 28;

    let people_3 = register_people(String::from("Algust"), 23);

    let people_4 = People { name: String::from("Jorge"), ..people_3 };


    let peoples = [people_1, people_2, people_3, people_4];
    for people in peoples {
        let people_name = people.name;
        println!("      - {people_name}");
    }

    println!("All peoples listed !")
}


fn register_people(name: String, age: i32) -> People {
    People { name, age }
}
