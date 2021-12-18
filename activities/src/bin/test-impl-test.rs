struct StudentRecord{
    maths: i32,
    science: i32,
    computer: i32,
}

impl StudentRecord {
    fn getRecord(&self) {
        println!("{:?}",self.maths);
        println!("{:?}",self.science);
        println!("{:?}",self.computer);
    }

    fn changemaths() -> Self{
        Self{maths:35,science:67,computer:88}
    }
}

fn main() {
    let kavin = StudentRecord{
        maths:100,
        science:99,
        computer:45,
    };
    let murali = StudentRecord{
        maths:90,
        science:95,
        computer:95,
    };

    //StudentRecord::getRecord(kavin);
    kavin.getRecord();
    murali.getRecord();
    let updated = StudentRecord::changemaths();
    updated.getRecord();
}