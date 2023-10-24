trait Duck {
    fn swim(&self);
    fn display(&self);
}
trait Quackable {
    fn quack(&self);
}

trait Flyable {
    fn fly(&self);
}

struct MallardDuck;
impl Duck for MallardDuck {
    fn swim(&self) {
        println!("swim");
    }
    fn display(&self) {
        println!("looks like a mallard");
    }
}
impl Quackable for MallardDuck {
    fn quack(&self) {
        println!("quack");
    }
}
impl Flyable for MallardDuck {
    fn fly(&self) {
        println!("Can't fly");
    }
}

struct RedheadDuck;
impl Duck for RedheadDuck {
    fn swim(&self) {
        println!("swim");
    }
    fn display(&self) {
        println!("looks like a redhead");
    }
}
impl Quackable for RedheadDuck {
    fn quack(&self) {
        println!("quack");
    }
}
impl Flyable for RedheadDuck {
    fn fly(&self) {
        println!("Can't fly");
    }
}

struct RubberDuck;
impl Duck for RubberDuck {
    fn swim(&self) {
        println!("swim");
    }
    fn display(&self) {
        println!("looks like a rubberduck");
    }
}
impl Quackable for RubberDuck {
    fn quack(&self) {
        println!("quack");
    }
}

struct DecoyDuck;
impl Duck for DecoyDuck {
    fn swim(&self) {
        println!("swim");
    }
    fn display(&self) {
        println!("looks like a decoyDuck");
    }
}

fn main() {
    println!("MallardDuck..");
    MallardDuck.quack();
    MallardDuck.swim();
    MallardDuck.display();
    MallardDuck.fly();

    println!("\nRedheadDuck..");
    RedheadDuck.quack();
    RedheadDuck.swim();
    RedheadDuck.display();
    RedheadDuck.fly();

    println!("\nRubberDuck..");
    RubberDuck.quack();
    RubberDuck.swim();
    RubberDuck.display();

    println!("\nRubberDuck..");
    DecoyDuck.swim();
    DecoyDuck.display();
}
