// stuct 
struct Chain {
    name: String,
    count: i32,
    id:Option<i32>,
}

fn print(_name: &str) {
    println!("name is {:?}",_name);

}

fn main() {
    let solana = Chain{
        name:"solana".to_owned(),
        count:45000,
        id:Some(3),
    };

    let eth = Chain{
        name:"ethereum".to_owned(),
        count:15000,
        id:None,
    };

    print(&solana.name);
    print(&eth.name);

    let test_vector = vec![
        Chain{
            name:"SOL".to_owned(),
            count:45000,
            id:Some(3),
        },
        Chain{
            name:"ETH".to_owned(),
            count:15000,
            id:None,
        },
    ];

    for i in test_vector{
        //println!("name -> {:?}",i.name);
        match i.id {
            Some(ans) => println!("some {:?}",ans),
            _ => println!("other {:?}",i.id),
        }
    }
}





