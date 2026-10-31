fn main() {
    let phrases = ["hello", "how", "are", "you", "my", "coder", "friend"]; // the count of size starts in 0
    let mut index = 0;

    while index < 6 {
        println!("{}", phrases[index]);

        index += 1;
    }
}
