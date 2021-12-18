// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

// * Use an enum for the tickets with data associated with each variant
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
enum Ticket {
    Backstage(String,i32),
    Vip(i32),
    Standard(String,i32)
}



fn main() {
    // * Create one of each ticket and place into a vector


    // let test_backstage = Ticket::Backstage("mine".to_owned(),100);
    // let test_standard = Ticket::Standard("standard".to_owned(),200);
    // let ticket_test = vec![test_backstage,test_standard];


    // or

    let sample_tickets = vec![Ticket::Backstage("backstage".to_owned(),10),
    Ticket::Standard("standard".to_owned(),20),
    Ticket::Vip(300)];

    for tickets in  sample_tickets {
        match tickets {
            Ticket::Backstage(holder,price) => println!("backstage ticket holder {:?} and price {:?}",holder,price),
            Ticket::Vip(price) => println!("vip price {:?}",price),
            Ticket::Standard(holder,price) => println!("standard ticket holder {:?} and price {:?}",holder,price),
        }
    }


}
