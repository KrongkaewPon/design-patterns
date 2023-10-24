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
    fn fly(&self) {
        println!("fly..");
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

struct RubberDuck;
impl DuckTrait for RubberDuck {
    fn display(&self) {
        println!("looks like a rubberduck");
    }
    fn quack(&self) {
        println!("Squeak");
    }
    fn fly(&self) {}
}
struct DecoyDuck;
impl DuckTrait for DecoyDuck {
    fn display(&self) {
        println!("looks like a DecoyDuck");
    }
    fn quack(&self) {}
    fn fly(&self) {}
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
    d.fly();

    println!("\nMallardDuck..");
    let d = MallardDuck {};
    d.display();
    d.quack();
    d.swim();
    d.fly();

    println!("\nRedheadDuck..");
    let d = RedheadDuck {};
    d.display();
    d.quack();
    d.swim();
    d.fly();

    println!("\nRubberDuck..");
    let d = RubberDuck {};
    d.display();
    d.quack();
    d.swim();
    d.fly();

    println!("\n DecoyDuck..");
    let d = DecoyDuck {};
    d.display();
    d.quack();
    d.swim();
    d.fly();
}
