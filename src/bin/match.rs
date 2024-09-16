// Example with boolean

// A match is like switch case
fn main() {
    let is_tall = true;
    let num: i32 = 3;

    match is_tall {
        true => println!("He's tall"),
        false => println!("Not tall"),
    }

    match num {
        1 => println!("One"),
        2 => println!("Two"),
        _ => println!("Other Number"),
    }
}
