struct Item{
    name: String,
    count: i32,
}

fn print(_name: &str){
    println!("{:?}",_name);
}

fn main () {
    // create a vector

    let sample_vector = vec![
        Item{
            name: "thanga".to_owned(),
            count: 10
        },
        Item{
            name: String::from("raj"),
            count:2
        }
    ];

    for at in sample_vector {
        print(&at.name);
        println!("and {:?}",at.count);
    }
}