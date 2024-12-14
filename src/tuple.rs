pub enum Access {
    Full,
}
pub fn one_two_three() -> (i32, String, Access) {
    (27, "Bowofade".to_owned(), Access::Full)
}

pub fn main() {
    let my_details = one_two_three();
    println!("{}", my_details.0)
}
