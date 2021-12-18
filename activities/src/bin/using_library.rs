
// tell if a vector has numbers present or not
// using standard library

// we are going to use is_empty()

// the above will return a bool (true if numbers are not present. and false if numbers are present)

fn main() {
    let num = vec![1,2,3,4,5];

    match num.is_empty(){
        true => println!(" is empty"),
        false => println!("numbers are present"),
    }

}

