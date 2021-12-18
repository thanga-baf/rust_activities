// Topic: Flow control using if..else if..else
//
// Program requirements:
// * Display ">5", "<5", or "=5" based on the value of a variable
//   is > 5, < 5, or == 5, respectively
//
// Notes:
// * Use a variable set to any integer value
// * Use an if..else if..else block to determine which message to display
// * Use the println macro to display messages to the terminal

fn main() {
    // * Use a variable set to any integer value
    let mut num = 7;
    if num == 6 {
        println!("its six")
    }else if num == 7 {
        println!("i")
    }else {
        println!("you might have found")
    }
    //num = 10;
    println!("result",num)

}
