fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    }; // this is amazing lmao, wtf i fave a loop in a variable

    println!("Result from first loop -> {result}");



    let mut another_count = 0;
    'counting_up: loop { // this is named loop label
        println!("count = {another_count}");
        let mut remaining = 14;

        loop {
            println!("remaining cicles = {remaining}");

            if remaining == 9 {
                break;
            }

            if another_count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        another_count += 1;
    }

    println!("Result from second loop -> {another_count}");


    let mut number = 9;

    while number != 0 {
        println!("{number}");

        number -= 1;
    }
    println!("LIFTOFF")
}
