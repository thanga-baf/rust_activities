// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Student{
    name: String,
    locker:Option<i32>,
}
fn main() {
    let sarvana = Student{
        name: "kumar".to_owned(),
        locker:Some(5),
    };
    println!("name {:?}",sarvana.name);
    match sarvana.locker{
        Some(_locker) => println!("locker details {:?}",_locker),
        None => println!("not present"),
    }
}
