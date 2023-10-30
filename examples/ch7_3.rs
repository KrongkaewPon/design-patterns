#[derive(Clone)]
struct Thermometer;
impl Thermometer {
    fn get_temperature(&self) -> f32 {
        34.5
    }
}

struct WeatherStation {
    thermometer: Thermometer,
}
impl WeatherStation {
    fn get_temperature(&self) -> f32 {
        self.thermometer.get_temperature()
    }
}
struct House {
    station: WeatherStation,
}
impl House {
    fn get_temp(&self) -> f32 {
        self.station.get_temperature()
    }
}
fn main() {
    let thermometer = Thermometer {};
    let station = WeatherStation { thermometer };
    let house = House { station };
    println!("{}", house.get_temp());
}
