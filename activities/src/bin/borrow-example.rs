struct Performance {
    count: i32,
    pages: i32,
}

fn get_count(_per:Performance){
    println!("{:?}",_per.count);
}

fn get_pages(_per:Performance) {
    println!("{:?}",_per.pages);
}

fn status(_temp:i32){
    println!("changes ------");
    println!("{:?}",_temp);
}


fn main() {
    let performance = Performance{
        count: 100,
        pages: 300,
    };
    println!("count/");
    get_count(performance);
    println!("pages");
    let changes=100;
    status(changes);
    status(changes);


    // if we add the below we will get error
    // value used after we move

    //we need to delete the value after we move
    //so we are commenting below
    //get_pages(performance);

}