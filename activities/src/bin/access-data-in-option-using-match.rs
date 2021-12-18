// strcut

struct Client{
    name: String,
    mail: Option<String>,
}

fn main() {
    let my_client = Client {
        name:"thanga".to_owned(),
        mail:Some("thanga@mail.com".to_owned()),
    };

    let ex_client = Client{
        name:"previous".to_owned(),
        mail:None,
    };

    // accessing using match

    match my_client.mail {
        Some(mail) => println!("mail is found {:?}",mail),
        _ => println!("nothing matches"),
    }

    match ex_client.mail{
        Some(mail) => println!("mail is found {:?}",mail),
        _ => println!("nothing matches"),
    }
}