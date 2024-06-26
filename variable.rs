fn main() {
    println!("hello world");

    // integer
    let age = 16;

    // double/float
    let my_balance = 345302.345;

    // string
    let my_name = "Bowofade Oyerinde";

    // boolean
    let _is_tall = false;

    // characeter
    let _cha = 'A';

    // the print statement must be a string
    println!("{age} is an integer");
    println!("My name is {my_name}");
    println!("The first character in the alphabet is {_cha}");

    // :? is for debug mode not for user
    println!("My account balance is {my_balance:?}");
}
