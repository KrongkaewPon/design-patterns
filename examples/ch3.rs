trait Beverage {
    fn cost(&self) -> f32;
    fn get_description(&self) -> String;
}

struct Espresso;
impl Espresso {
    fn new() -> Box<dyn Beverage> {
        Box::new(Espresso)
    }
}
impl Beverage for Espresso {
    fn cost(&self) -> f32 {
        1.99
    }
    fn get_description(&self) -> String {
        "Espresso".to_string()
    }
}

struct HouseBlend;
impl Beverage for HouseBlend {
    fn cost(&self) -> f32 {
        0.89
    }
    fn get_description(&self) -> String {
        "HouseBlend".to_string()
    }
}

trait CondimentDecorator: Beverage {
    fn new(beverage: Box<dyn Beverage>) -> Box<dyn Beverage>;
}

struct Mocha {
    beverage: Box<dyn Beverage>,
}

impl CondimentDecorator for Mocha {
    fn new(beverage: Box<dyn Beverage>) -> Box<dyn Beverage> {
        Box::new(Mocha { beverage })
    }
}

impl Beverage for Mocha {
    fn cost(&self) -> f32 {
        self.beverage.cost() + 0.2
    }
    fn get_description(&self) -> String {
        format!("{}, Mocha", self.beverage.get_description())
    }
}

// Espresso 1.99
// DarkRoast
// HouseBlend 0.89
// Decaf

// Milk
// Mocha 0.20
// Soy
// Whip

fn main() {
    println!("\n ========");
    // let mut beverage: Box<dyn Beverage> = Box::new(Espresso);
    // println!("cost {:?} ", beverage.cost());
    // println!("description {:?} ", beverage.get_description());
    // beverage = Box::new(Mocha { beverage });
    // println!("cost {:?} ", beverage.cost());
    // println!("description {:?} ", beverage.get_description());

    let mut beverage = Espresso::new();
    beverage = Mocha::new(beverage);
    println!("cost {:?} ", beverage.cost());
    println!("description {:?} ", beverage.get_description());
    beverage = Mocha::new(beverage);
    println!("cost {:?} ", beverage.cost());
    println!("description {:?} ", beverage.get_description());
}
