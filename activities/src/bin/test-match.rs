
fn main() {
    let temp = "testing match ";

    match temp {
        "testing ?" => println!("still testing"),
        "testing match" => println!("yesy"),
        _ => println!("no"),
    }
}