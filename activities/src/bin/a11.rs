// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter


struct Grocery{
    quantity: i32,
    id: i32,
}

fn quantity(_qty: &Grocery) {
    println!("{:?}",_qty.quantity);
}

fn id(_id: &Grocery) {
    println!("{:?}",_id.id);

}

fn main() {
    let apple = Grocery {
        quantity: 100,
        id: 332,
    };
    quantity(&apple);
    id(&apple);
}

// if we didn't use &, then the function will take ownership
// once the function gets completed, it will be dropped
  