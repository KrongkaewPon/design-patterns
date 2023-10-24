struct Light;
impl Light {
    fn on(&self) {
        println!("light is on")
    }
    fn off(&self) {
        println!("light is off")
    }
}

struct LightOnCommand {
    light: Light,
}

impl LightOnCommand {
    fn new(light: Light) -> Self {
        Self { light }
    }
}

struct GarageDoor;
impl GarageDoor {
    fn open(&self) {
        println!("GarageDoor is open")
    }
    fn close(&self) {
        println!("GarageDoor is close")
    }
}

struct GarageDoorOpenCommand {
    open: GarageDoor,
}

impl GarageDoorOpenCommand {
    fn new(open: GarageDoor) -> Self {
        Self { open }
    }
}

trait Command {
    fn execute(&mut self);
}

impl Command for LightOnCommand {
    fn execute(&mut self) {
        self.light.on();
    }
}
impl Command for GarageDoorOpenCommand {
    fn execute(&mut self) {
        self.open.open();
    }
}

struct RemoteControl {
    slot: Option<Box<dyn Command>>,
}

impl RemoteControl {
    fn new() -> Self {
        Self { slot: None }
    }
    fn set_command(&mut self, command: Box<dyn Command>) {
        self.slot = Some(command)
    }
    fn button_was_pressed(&mut self) {
        self.slot.as_mut().unwrap().execute()
    }
}

fn main() {
    let mut remote = RemoteControl::new();
    let light = Light {};
    let light_on = LightOnCommand::new(light);

    let garage_door = GarageDoor {};
    let garage_door_open = GarageDoorOpenCommand::new(garage_door);

    remote.set_command(Box::new(light_on));
    remote.button_was_pressed();

    remote.set_command(Box::new(garage_door_open));
    remote.button_was_pressed();
}
