//Topic: Data management using tuples
fn cartesian() -> (f32, f32) {
    (23.9790, 5.0)
}
fn main() {
    let (x, y) = cartesian();

    if y > 5f32 {
        println!("greater than 5")
    } else if y < 5f32 {
        println!("less than 5")
    } else {
        println!("=5")
    }
}
