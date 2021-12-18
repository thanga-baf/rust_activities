struct Survey {
    q1:Option<i32>,
    q2:Option<bool>,
    q3:Option<String>,
}

fn main() {
    let thanga_response = Survey{
        q1:Some(2),
        q2:None,
        q3:Some("some-data".to_owned()),
    };

    match thanga_response.q1 {
        Some(_data) => println!("present. the data is {:?}",_data),
        None => println!("not present any"),
    }
    match thanga_response.q2 {
        Some(_data) => println!("present. the data is {:?}",_data),
        None => println!("not present any"),
    }
    match thanga_response.q3 {
        Some(_data) => println!("present. the data is {:?}",_data),
        None => println!("not present any"),
    }

}