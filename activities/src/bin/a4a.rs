// Topic: Decision making with match
//
// Program requirements:
// * Display "it's true" or "it's false" based on the value of a variable
//
// Notes:
// * Use a variable set to either true or false
// * Use a match expression to determine which message to display

fn main() {
    // * Use a variable set to either true or false
    let temp = true;
    // * Use a match expression to determine which message to display
    match temp {
        true => println!("its true"),
        false => println!("its false"),
        _ => println!("unable to predict"),
    }
}
