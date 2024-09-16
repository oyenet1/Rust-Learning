// write a function to add two numbers together

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn display_result(result: i32) {
    println!("The result is: {:?}", result);
}

fn main() {
    display_result(add(157, 90));
}
