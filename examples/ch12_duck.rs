use once_cell::sync::Lazy;
use std::sync::Mutex;

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

struct QuackCounter {
    quack: Box<dyn Quackable>,
    number_of_quacks: i32,
}

impl QuackCounter {
    fn new(quack: Box<dyn Quackable>) -> Self {
        Self {
            quack,
            number_of_quacks: 0,
        }
    }

    fn quack(&mut self) {
        self.quack.quack();
        self.number_of_quacks += 1;
        *GLOBAL.lock().unwrap() += 1;
    }

    fn get_quack(&self) -> i32 {
        self.number_of_quacks
    }
}

static GLOBAL: Lazy<Mutex<i32>> = Lazy::new(|| Mutex::new(0));

fn main() {
    let mut counter = QuackCounter::new(Box::new(MallardDuck {}));
    counter.quack();
    counter.quack();
    println!("number_of_quacks: {}", counter.get_quack());

    let mut counter2 = QuackCounter::new(Box::new(RedheadDuck {}));
    counter2.quack();
    println!("number_of_quacks: {}", counter2.get_quack());
    println!("GLOBAL: {}", *GLOBAL.lock().unwrap());
}
