//Topic: Organising similar data using struct
#[derive(Debug)]
enum Flavor {
    StrawBerry,
    Banana,
    Vanilla,
}

struct Drink {
    flavor: Flavor,
    ounce: i32,
}

fn print_drink(drink: Drink) {
    match drink.flavor {
        Flavor::Banana => print!(
            "The flavour is {:?} and the ouce is {:?}",
            drink.flavor, drink.ounce
        ),
        Flavor::Vanilla => print!(
            "The flavour is {:?} and the ouce is {:?}",
            drink.flavor, drink.ounce
        ),
        Flavor::StrawBerry => print!(
            "The flavour is {:?} and the ouce is {:?}",
            drink.flavor, drink.ounce
        ),
    }
}

fn main() {
    let drink = Drink {
        flavor: Flavor::Vanilla,
        ounce: 42,
    };

    print_drink(drink);
}
