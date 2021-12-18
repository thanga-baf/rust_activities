struct Performance {
    count: i32,
    pages: i32,
}

fn get_count(_per:&Performance){
    println!("{:?}",_per.count);
}

fn get_pages(_per:&Performance) {
    println!("{:?}",_per.pages);
}


fn main() {
    let performance = Performance{
        count: 100,
        pages: 300,
    };
    println!("count/");
    get_count(&performance);
    println!("pages");
    get_pages(&performance);

}