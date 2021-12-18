
enum Check {
    Check,
}

fn one_two_three() -> (i32, i32, i32){
    (1,2,3)
}

fn main() {
    let numbers = one_two_three();
    println!("printing the whole");
    println!("{:?}",numbers);

    let (a,b,c) = one_two_three();
    println!("printing the values");
    println!("{:?}",a);
    println!("{:?}",b);


    println!("printing the demo");
    // println!("{:?}",demoname)
    // println!("{:?}",acceess)
}