trait DuckTrait {
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

enum QuackType {
    quack(Quack),
    squeak(Squeak),
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

struct Duck {
    fly_behavior: Box<dyn FlyBehavior>,
    quack_behavior: QuackType,
}
impl Duck {
    fn perform_quack(&self) {
        match &self.quack_behavior {
            QuackType::quack(q) => q.quack(),
            QuackType::squeak(q) => q.quack(),
        }
    }
    fn perform_fly(&self) {
        self.fly_behavior.fly();
    }
    fn set_fly_behavior(&mut self, a: impl FlyBehavior + 'static) {
        self.fly_behavior = Box::new(a)
    }
}

fn main() {
    println!("Duck..");
    let mut d = Duck {
        fly_behavior: Box::new(FlyWithWings),
        quack_behavior: QuackType::quack(Quack {}),
    };
    d.perform_fly();
    d.set_fly_behavior(FlyNoWay);
    d.perform_fly();
    d.perform_quack();
}
