struct WeatherData {
    temp: f64,
    humidity: f64,
    pressure: f64,
}

impl WeatherData {
    fn new(temp: f64, humidity: f64, pressure: f64) -> Self {
        WeatherData {
            temp,
            humidity,
            pressure,
        }
    }
}

#[derive(Default)]
struct CurrentConditionsDisplay {
    temp: f64,
    humidity: f64,
    pressure: f64,
}
impl CurrentConditionsDisplay {
    fn update(&self, temp: f64, humidity: f64, pressure: f64) {}
}

#[derive(Default)]
struct StatisticsDisplay {
    temp: f64,
    humidity: f64,
    pressure: f64,
}
impl StatisticsDisplay {
    fn update(&self, temp: f64, humidity: f64, pressure: f64) {}
}

#[derive(Default)]
struct ForecastDisplay {
    temp: f64,
    humidity: f64,
    pressure: f64,
}
impl ForecastDisplay {
    fn update(&mut self, temp: f64, humidity: f64, pressure: f64) {
        self.temp = temp;
        self.humidity = humidity;
        self.pressure = pressure;
    }

    pub fn display(&self) {
        println!(
            "ForecastDisplay: {} {} {}",
            self.temp, self.humidity, self.pressure
        )
    }
}

fn main() {
    println!("Set Weather Data..");
    let weather_data = WeatherData::new(10.0, 20.0, 30.0);

    let mut forecast_display = ForecastDisplay::default();
    forecast_display.update(
        weather_data.temp,
        weather_data.humidity,
        weather_data.pressure,
    );

    forecast_display.display();
}
