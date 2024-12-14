// Topic: Working with an enum
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

fn main() {
    print_color(Color::Blue);
    print_color(Color::Green);
    print_color(Color::Red);
}
