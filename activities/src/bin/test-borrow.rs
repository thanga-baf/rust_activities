enum Process {
    Completed,
    OnProcess,
    WithHeld,
}

fn status(_try: Process) {
    match _try{
        Process::Completed => println!("completed"),
        Process::OnProcess => println!("OnProcess"),
        Process::WithHeld => println!("WithHeld"),
    }
}

fn main() {

    let test = Process::Completed;
    status(test);
    status(test);

}