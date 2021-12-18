// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print


// * Use a function that returns a tuple

fn temp_func() -> (i32, i32){
    (12, 13)
}
fn main() {
    let test = temp_func();
    println!("{:?}",test);

    let (a, b) = temp_func();
    if a < 10 {
        println!("its a ");
    }else  {
        println!("its b");
        println!("{:?}",b);
    }


}
