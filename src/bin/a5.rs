// Topic: Decision making with match

// Display "One", "Two", "Three" based on the value the person pass

fn main() {
    let mut num = 1;

    loop {
        print!("{:?} ", num);

        if num == 4 {
            break;
        }

        num += 1;
    }
}
