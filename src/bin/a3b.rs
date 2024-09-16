// Topic: Flow control using if  ...else

// Display equals to ">5" or "<5" or "=5" based on the value the person pass

fn display(num: i32) {
    if num > 5 {
        println!(">5");
    } else if num < 5 {
        println!("<5");
    } else {
        println!("=5");
    }
}

fn main() {
    let num: i32 = 8;

    display(num);
}
