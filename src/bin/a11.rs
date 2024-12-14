// Topic: Ownership

struct GroceryItem {
    id: i32,
    quantity: i32,
}

fn print_quantity(item: &GroceryItem) {
    println!("quantity: {}", item.quantity);
}

fn print_id(item: &GroceryItem) {
    println!("id: {}", item.id);
}

fn main() {
    let apple = GroceryItem {
        id: 1,
        quantity: 34,
    };

    print_id(&apple);
    print_quantity(&apple);
}
