fn main() {
    println!("Here 1");

    my_custom_function();

    my_custom_param_function(3);

    let returned_value = four();
    println!("Here {returned_value}");
}


fn my_custom_function() {
    println!("Here 2");
}

fn my_custom_param_function(number: i32) {
    println!("Here {number}")
}

fn four() -> i32 { // function with return
    4
}
