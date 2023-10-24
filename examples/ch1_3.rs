trait Duck {
    fn swim(&self);
    fn display(&self);
    fn perform_quack(&self);
    fn perform_fly(&self);
}

trait FlyBehavior {
    fn fly(&self);
}
struct FlyWithWings;
impl FlyBehavior for FlyWithWings {
    fn fly(&self) {
        println!("duck flying");
    }
}
struct FlyNoWay;
impl FlyBehavior for FlyNoWay {
    fn fly(&self) {
        println!("can’t fly!");
    }
}

trait QuackBehavior {
    fn quack(&self);
}
struct Quack;
impl QuackBehavior for Quack {
    fn quack(&self) {
        println!("duck quacking");
    }
}
struct Squeak;
impl QuackBehavior for Squeak {
    fn quack(&self) {
        println!("duckie squeak");
    }
}
struct MuteQuack;
impl QuackBehavior for MuteQuack {
    fn quack(&self) {
        println!("can’t quack!");
    }
}

struct MallardDuck {
    fly_behavior: Box<dyn FlyBehavior>,
    quack_behavior: Box<dyn QuackBehavior>,
}
impl Duck for MallardDuck {
    fn swim(&self) {
        println!("swim");
    }
    fn display(&self) {
        println!("looks like a mallard");
    }
    fn perform_quack(&self) {
        self.quack_behavior.quack();
    }
    fn perform_fly(&self) {
        self.fly_behavior.fly();
    }
}

struct RedheadDuck {
    fly_behavior: FlyWithWings,
    quack_behavior: Quack,
}
impl Duck for RedheadDuck {
    fn swim(&self) {
        println!("swim");
    }
    fn display(&self) {
        println!("looks like a redhead");
    }
    fn perform_quack(&self) {
        self.quack_behavior.quack();
    }
    fn perform_fly(&self) {
        self.fly_behavior.fly();
    }
}

struct RubberDuck {
    fly_behavior: FlyWithWings,
    quack_behavior: Quack,
}
impl Duck for RubberDuck {
    fn swim(&self) {
        println!("swim");
    }
    fn display(&self) {
        println!("looks like a rubberduck");
    }
    fn perform_quack(&self) {
        self.quack_behavior.quack();
    }
    fn perform_fly(&self) {
        self.fly_behavior.fly();
    }
}

struct DecoyDuck {
    fly_behavior: FlyWithWings,
    quack_behavior: Quack,
}
impl Duck for DecoyDuck {
    fn swim(&self) {
        println!("swim");
    }
    fn display(&self) {
        println!("looks like a decoyDuck");
    }
    fn perform_quack(&self) {
        self.quack_behavior.quack();
    }
    fn perform_fly(&self) {
        self.fly_behavior.fly();
    }
}

fn main() {
    println!("MallardDuck..");
    let a = MallardDuck {
        fly_behavior: FlyWithWings,
        quack_behavior: Quack,
    };
    // MallardDuck.quack();
    // MallardDuck.swim();
    // MallardDuck.display();
    // MallardDuck.fly();

    // println!("\nRedheadDuck..");
    // RedheadDuck.quack();
    // RedheadDuck.swim();
    // RedheadDuck.display();
    // RedheadDuck.fly();

    // println!("\nRubberDuck..");
    // RubberDuck.quack();
    // RubberDuck.swim();
    // RubberDuck.display();

    // println!("\nRubberDuck..");
    // DecoyDuck.swim();
    // DecoyDuck.display();
}
