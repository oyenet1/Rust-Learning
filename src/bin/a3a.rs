// Topic: Flow control using if else

// display a message base on boolean value,
// if true display "Hello"
// if false display "Goodbye"

fn greet_or_bye(is_greet: bool) {
    if is_greet {
        println!("Hello");
    } else {
        println!("Goodbye");
    }
}

fn main() {
    greet_or_bye(false);
}
