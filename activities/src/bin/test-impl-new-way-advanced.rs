struct Temprature{
    degree: f64
}

//implementation block
impl Temprature{
    //we dont need to pass parameter. we can use &self
    fn get(&self) {
        println!("degree is {:?}",self.degree);
    }

    //lets add one more function
    fn cold() -> Self {
        Self{degree:10.0}
    }

    fn addmore() -> Self {
        Self { degree: 50.0 }
    }
}

fn main() {
    let pass = Temprature{
        degree:100.0
    };

    let change = Temprature{
        degree:102.0   
    };
    //we can't call like below once we put function inside impl
    //get(pass);

    //we need to call like below
    // Temprature::get(&pass);
    // Temprature::get(&change);

    // we can call the function directly using dot variable
    pass.get();
    change.get();

    let cold = Temprature::cold();
    cold.get();
    let mild = Temprature::addmore();
    mild.get();
}