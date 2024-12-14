// Topic: Working with an enum

// Display "One", "Two", "Three" based on the value the person pass

fn main() {
    enum Color {
        Red,
        Green,
        Blue,
    }

    fn print_color(color: Color) {
        match color {
            Color::Red => println!("Red"),
            Color::Green => println!("Green"),
            Color::Blue => println!("Blue"),
        }
    }

    print_color(Color::Blue);
}
