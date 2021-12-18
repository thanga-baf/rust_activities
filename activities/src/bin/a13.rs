// Topic: Vectors
//
// Requirements:
// * Print 10, 20, "thirty", and 40 in a loop
// * Print the total number of elements in a vector
//
// Notes:
// * Use a vector to store 4 numbers
// * Iterate through the vector using a for..in loop
// * Determine whether to print the number or print "thirty" inside the loop
// * Use the .len() function to print the number of elements in a vector

fn main() {
    let test_vector = vec![10,20,30,40];

    for test in &test_vector{

        match test{
            30 => println!("thirty"),
            _ => println!("answer = {:?}",test),
        }
        //println!("without match answer = {:?}",test);
    }

    // printing the length
    println!("length of the vector {:?}",&test_vector.len());
}
