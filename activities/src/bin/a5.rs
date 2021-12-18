// Topic: Looping using the loop statement
//
// Program requirements:
// * Display "1" through "4" in the terminal
//
// Notes:
// * Use a mutable integer variable
// * Use a loop statement
// * Print the variable within the loop statement
// * Use break to exit the loop

fn main() {
    let mut temp = 10;

    loop {
        temp = temp - 1;
        if temp == 4 {
            println!("{:?}",temp);
            break
        }
    }
}
