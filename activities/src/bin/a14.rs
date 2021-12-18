// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:

// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

// * Use a struct for a persons age, name, and favorite color
struct Person {
    age: i32,
    name: String,
    color: String,
}

fn print_name(_name: &str){
println!("name {:?}",_name);
}

fn print_color(_color: &str){
    println!("color {:?}",_color);
}


fn main() {
    let person_vector = vec![
        Person{
            age:32,
            name:"thanga".to_owned(),
            color:"orange".to_owned(),
        },
        Person{
            age:33,
            name:String::from("raj"),
            color:String::from("red"),
        }
    ];

    for values in person_vector {
        print_name(&values.name);
        print_color(&values.color);
        println!("and {:?}",values.age);
    }
}
