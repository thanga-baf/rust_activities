// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

// * Use a function to print the messages
fn print_the_msg(test:bool){
    match test{
        true => println!("its true"),
        false => println!("its flase"),
    }
}

fn main() {
    // * Use a boolean variable set to the result of
    let booltest = 100;
    let is_true = booltest > 100;
    if is_true == true {
        println!("true")
    }else {
        println!("flase")
    }
    print_the_msg(is_true);
}
