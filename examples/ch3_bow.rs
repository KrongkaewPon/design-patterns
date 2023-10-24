trait Beverage {
    fn cost(&self) -> f32;
    fn get_description(&self) -> String;
}

struct Espresso;
impl Beverage for Espresso {
    fn cost(&self) -> f32 {
        1.99
    }
    fn get_description(&self) -> String {
        "Espresso".to_string()
    }
}
struct Mocha {
    beverage: Box<dyn Beverage>,
}

// trait CondimentDecorator: Beverage {
//     fn get_description(&self) -> String;
// }

// impl CondimentDecorator for Mocha {
//     fn get_description(&self) -> String {
//         format!("{}, Mocha", self.beverage.get_description())
//     }
// }

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
    let beverage: Box<Espresso> = Box::new(Espresso);
    println!("\n ========");
    println!("cost {:?} ", beverage.cost());
    println!("description {:?} ", beverage.get_description());

    //
    let mut beverage: Box<dyn Beverage> = Box::new(Espresso);
    println!("\n ========");
    println!("cost {:?} ", beverage.cost());
    println!("description {:?} ", beverage.get_description());
    beverage = Box::new(Mocha { beverage });
    println!("cost {:?} ", beverage.cost());
    println!("description {:?} ", beverage.get_description());
}
