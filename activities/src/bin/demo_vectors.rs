

// below is a vector of same pieces

// traverse is happening below

// fn main() {
//     let num = vec![1,2,3,4,5,6];

//     for i in num {
//         println!("{:?}",i);
//     }
// }












// using a struct

struct Test {
    mark: i32,
}

fn main() {

    let init_vector = vec![
        Test{mark:100},
        Test{mark:90},
        Test{mark:80},
        Test{mark:70},
    ];

    for marks in init_vector {
        println!("mark is = {:?}",marks.mark);
    }
}