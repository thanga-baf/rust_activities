use std::sync::Mutex;

fn main() {
    let my_mutex = Mutex::new(15);
    println!("{:?}",my_mutex);
    let mut mut_changer = my_mutex.lock().unwrap();
    println!("{:?}",my_mutex);
    *mut_changer = 7;
    std::mem::drop(mut_changer);
    println!("{:?}",my_mutex);
}