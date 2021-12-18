struct Temprature{
    degree: f64
}

//implementation block
impl Temprature{
    //we dont need to pass parameter. we can use &self
    fn get(&self) {
        println!("degree is {:?}",self.degree);
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
    Temprature::get(&pass);
    Temprature::get(&change);

}