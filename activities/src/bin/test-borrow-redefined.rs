//rust uses ownership model
//owner of the model is responsible for cleaning up the memory
//memory can either be removed or borrowed


enum Process {
    Completed,
    OnProcess,
    WithHeld,
}

fn status(_try: &Process) {
    match _try{
        Process::Completed => println!("completed"),
        Process::OnProcess => println!("OnProcess"),
        Process::WithHeld => println!("WithHeld"),
    }
}

fn main() {

    let test = Process::Completed;
    //owner is responsible for cleaning up the memory
    //memory can be borrowed or moved
    status(&test);
    status(&test);

}