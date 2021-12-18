// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor


// * Use an enum to create different flavors of drinks
// enum Drinks {
//     Coke,
//     Mango,
//     Apple,
//     Milk
// }

// struct Drinkdetails {
//     flavor: i32,
//     ounces: i32,
// }


// fn main() {

//     let maza = Drinkdetails {
//         flavor: 10,
//         ounces: 20,
//     };

//     let _flavor = maza.flavor;
//     let _ounces = maza.ounces;

//     fn get_the_flavor() {
//         println!("{:?}",_flavor);
//         println!("{:?}",_ounces);   
//     }

//     let test_drinks = Drinks::Mango;

//     match test_drinks {
//         Drinks::Mango => println!("its mango"),
//         Drinks::Coke => println!("its coke"),
//         Drinks::Apple => println!("its apple"),
//         Drinks::Milk => println!("its milk"),
//     }


// }

enum Flavor {
    Sweety,
    Sour,
    Sparkling,
    Spicy,
}

struct Drink {
    flavor: Flavor,
    ounces: f64,
}

fn print_drink(drink: Drink){
    match drink.flavor{
        Flavor:: Sweety => println!("it is sweet"),
        Flavor:: Sour => println!("it is sour"),
        Flavor:: Sparkling => println!("it is sparkling"),
        Flavor:: Spicy => println!("it is spicy"),
    }

    //printing ounces
    println!("{:?}",drink.ounces);

}

fn main() {
    let sweet = Drink {
        flavor: Flavor::Sweety,
        ounces: 20.0
    };

    println!("below is for sweet");
    //println!(sweet);
    print_drink(sweet);


    let spicy = Drink{
        flavor: Flavor::Spicy,
        ounces: 10.0
    };

    println!("below is spicy");
    //println!(spicy);
    print_drink(spicy);

}
