fn main() {
    let mut test = 25;
    loop {
        println!("{:?}",test);
        //test -= 1;
        test = test-1;
        println!("{:?}",test);

        if test == 10 {
            println!("reached the limit");
            break;
        }
    }
}