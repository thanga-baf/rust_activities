// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics


enum Color{
    Red,
    Green,
    Yellow,
}


struct Dimension {
    width:i32,
    height:i32,
    depth:i32,
}

impl Color {
    fn print(&self){
        match self{
            Color::Red => println!("Its Red"),
            Color::Green => println!("Its Green"),
            Color::Yellow => println!("Its Yellow"),
        }
    }
}


impl Dimension {
    fn print(&self){
        println!("height {:?}",self.height);
        println!("width {:?}",self.width);
        println!("depth {:?}",self.depth);
    }
}

// * Use a struct to encapsulate the box characteristics
struct ShippingBox {
    color: Color,
    weight: f64,
    dimensions:Dimension,
}

// self always refers to whatever we are implementing
// below self refers to shippingBox
impl ShippingBox {
    fn new(weight:f64,color:Color,dimensions:Dimension) -> Self {
        Self{
            color,
            weight,
            dimensions,
        }
    }

    fn print(&self){
        self.color.print();
        self.dimensions.print();
        println!("weight {:?}",self.weight);
    }
}

fn main() {

    // create a small box

    // first add dimensions
    let my_test_box = Dimension{
        width: 5,
        height: 5,
        depth: 5,
    };

    // we can use below way rather than creating a new struct

    let my_shipping_box = ShippingBox::new(10.0,Color::Red,my_test_box);
    my_shipping_box.print();

}
