// enum 
enum Discount{
    Percent(i32),
    Flat(i32),
}

// struct
struct Ticket {
    event: String,
    price: i32,
}

fn main() {
    let define = 4;
    match define {
        3 => println!("its three"),
        other => println!("non {:?}",other),
    }

    // creating a flat
    let flat = Discount::Flat(2);

    match flat {
        Discount::Flat(2) => println!("flat discount is 2"),
        Discount::Flat(amount) => println!("discount rate is {:?}",amount),
        _ => println!("nothin matches"),
    }

    // defining a struct

    let test_struct = Ticket {
        event: "thanga".to_owned(),
        price: 20,
    };

    match test_struct{
        Ticket{price:10, ..} => println!("event when price is at 10 "), 
        Ticket{price:20,event} => println!("price and event => {:?} ",event),
        _ => println!("nothing matches"),
    }
}