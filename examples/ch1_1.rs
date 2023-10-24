trait Duck {
    fn quack(&self) {
        println!("quack");
    }
    fn swim(&self) {
        println!("swim");
    }
    fn display(&self);
    fn fly(&self);
}

struct MallardDuck;
impl Duck for MallardDuck {
    fn display(&self) {
        println!("looks like a mallard");
    }
    fn fly(&self) {
        println!("Can fly");
    }
}

struct RedheadDuck;
impl Duck for RedheadDuck {
    fn quack(&self) {
        println!("quack");
    }
    fn swim(&self) {
        println!("swim");
    }
    fn display(&self) {
        println!("looks like a redhead");
    }
    fn fly(&self) {
        println!("Can't fly");
    }
}

struct RubberDuck;
impl Duck for RubberDuck {
    fn quack(&self) {
        println!("Squeak");
    }
    fn swim(&self) {
        println!("swim");
    }
    fn display(&self) {
        println!("looks like a rubberduck");
    }
    fn fly(&self) {
        println!("Can't fly");
    }
}

fn main() {
    println!("MallardDuck..");
    MallardDuck.quack();
    MallardDuck.swim();
    MallardDuck.display();

    println!("\nRedheadDuck..");
    RedheadDuck.quack();
    RedheadDuck.swim();
    RedheadDuck.display();

    println!("\nRubberDuck..");
    RubberDuck.quack();
    RubberDuck.swim();
    RubberDuck.display();
    RubberDuck.fly();
}
