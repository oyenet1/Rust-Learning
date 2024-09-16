// Topic: Decision making with match

// Display "One", "Two", "Three" based on the value the person pass

fn main() {
    let num = 5;
    match num {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Other Number"),
    }
}
