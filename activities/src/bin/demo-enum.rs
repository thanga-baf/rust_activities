enum Decide {
    Correct,
    Wrong,
}

enum Test {
    Pass,
    Fail,
}

fn nope() {
    println!("calling wrong function")
}

fn yes(){
    println!("yes cleared")
}

fn main() {
    let answer = Decide::Correct;
    let tryonemore = Test::Fail;
    let trytopass = Test::Pass;

    match answer {
        Decide::Correct => println!("it is correct lets call the function correct"),
        Decide::Wrong => println!("it is wrong"),
    }

    println!("trying to use the enum");
    match tryonemore {
        Test::Fail => nope(),
        Test::Pass => yes(),
    }

    println!("using same enum here");
    match trytopass {
        Test::Fail => nope(),
        Test::Pass => yes(),
    }
}