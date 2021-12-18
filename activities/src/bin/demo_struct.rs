struct Case {
    range: i32,
    categeory: i32,
}



fn main() {
    let new_Arrival = Case {
        range: 10,
        categeory: 1,
    };
    //assigning the range
    let newarrival_range = new_Arrival.range;
    let newarrival_category = new_Arrival.categeory;
    println!("value");
    println!("{:?}",newarrival_range);
    println!("range");
    println!("{:?}",newarrival_category);
}