//Topic: Working with expression
fn print_ex(x: bool) -> () {
    match x {
        true => println!("> 100"),
        false => println!("< 100"),
    };
}
fn main() {
    let value = 05;
    let is_big = value > 100;
    print_ex(is_big);
}
