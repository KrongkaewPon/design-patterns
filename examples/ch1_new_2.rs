struct Duck {}
trait DuckTrait {
    fn quack(&self) {
        println!("quack");
    }
    fn swim(&self) {
        println!("swim");
    }
    fn display(&self) {
        println!("looks like a duck");
    }
}
impl DuckTrait for Duck {}

struct MallardDuck;
impl DuckTrait for MallardDuck {
    fn display(&self) {
        println!("looks like a mallard");
    }
}

struct RedheadDuck;
impl DuckTrait for RedheadDuck {
    fn display(&self) {
        println!("looks like a redhead");
    }
}

// struct RubberDuck;
// impl Duck for RubberDuck {
//     fn quack(&self) {
//         println!("Squeak");
//     }
//     fn swim(&self) {
//         println!("swim");
//     }
//     fn display(&self) {
//         println!("looks like a rubberduck");
//     }
//     fn fly(&self) {
//         println!("Can't fly");
//     }
// }

fn main() {
    let d = Duck {};
    d.display();
    d.quack();
    d.swim();

    println!("\nMallardDuck..");
    let d = MallardDuck {};
    d.display();
    d.quack();
    d.swim();

    println!("\nRedheadDuck..");
    let d = RedheadDuck {};
    d.display();
    d.quack();
    d.swim();
    // println!("\nRubberDuck..");
    // RubberDuck.quack();
    // RubberDuck.swim();
    // RubberDuck.display();
    // RubberDuck.fly();
}
