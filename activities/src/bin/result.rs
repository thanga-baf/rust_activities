enum Menuchoice {
    Mainmenu,
    Start,
    Quit
}

fn get_choice(_input: &str) -> Result<Menuchoice,String>{
    match _input {
        "mainmenu" => Ok(Menuchoice::Mainmenu),
        "start" => Ok(Menuchoice::Start),
        "quit" => Ok(Menuchoice::Quit),
        _ => Err("menu choice out of bound".to_owned()),
    }
}

// fn print_choice(_input:&Menuchoice){
//     println!("choice {:?}",_input);
// }

fn main() {
    let mychoice = get_choice("mainmenu");
    println!("choice {:?}",mychoice);
    
    // below won't work

    //mychoice.print_choice();

    //print_choice(&mychoice);
}