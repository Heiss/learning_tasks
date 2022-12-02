use std::{cell::RefCell, cmp::PartialOrd, ops::Deref, rc::Rc};

use crate::observer::{DisplayElement, Subject, WeatherData};

pub trait Observer {
    fn update(&mut self, temp: f32, humidity: f32, pressure: f32);
}

impl PartialEq for dyn Observer {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

pub struct CurrentConditionsDisplay {
    temperature: f32,
    humidity: f32,
    #[allow(dead_code)]
    weather_data: Rc<RefCell<WeatherData>>,
}

impl CurrentConditionsDisplay {
    #[allow(dead_code)]
    pub fn new(wd: Rc<RefCell<WeatherData>>) -> Rc<RefCell<dyn Observer>> {
        let display = Rc::new(RefCell::new(CurrentConditionsDisplay {
            humidity: 0.,
            temperature: 0.,
            weather_data: wd.clone(),
        }));

        wd.deref().borrow_mut().register_observer(display.clone());

        display
    }
}

impl Observer for CurrentConditionsDisplay {
    fn update(&mut self, temp: f32, humidity: f32, _pressure: f32) {
        self.temperature = temp;
        self.humidity = humidity;
        self.display();
    }
}

impl DisplayElement for CurrentConditionsDisplay {
    fn display(&self) {
        println!(
            "Current conditions: {}F degreses and {}% humidty",
            self.temperature, self.humidity
        );
    }
}

pub struct StatisticsDisplay {
    min_temp: f32,
    temp_sum: f32,
    num_readings: f32,
    max_temp: f32,
    #[allow(dead_code)]
    weather_data: Rc<RefCell<WeatherData>>,
}

impl StatisticsDisplay {
    #[allow(dead_code)]
    pub fn new(wd: Rc<RefCell<WeatherData>>) -> Rc<RefCell<dyn Observer>> {
        let display = Rc::new(RefCell::new(StatisticsDisplay {
            max_temp: 0.0,
            min_temp: 200.0,
            temp_sum: 0.0,
            num_readings: 0.0,
            weather_data: wd.clone(),
        }));

        wd.deref().borrow_mut().register_observer(display.clone());

        display
    }
}

impl Observer for StatisticsDisplay {
    fn update(&mut self, temp: f32, _humidity: f32, _pressure: f32) {
        self.temp_sum += temp;
        self.num_readings += 1.0;

        self.max_temp = temp.max(self.max_temp);
        self.min_temp = temp.min(self.min_temp);

        self.display();
    }
}

impl DisplayElement for StatisticsDisplay {
    fn display(&self) {
        println!(
            "Avg/Max/Min temperature = {}/{}/{}",
            self.temp_sum / self.num_readings,
            self.max_temp,
            self.min_temp
        );
    }
}

struct Temperature(f32);
struct Humidity(f32);

#[allow(dead_code)]
struct Pressure(f32);

pub struct ThirdPartyDisplay {
    temperature: Temperature,
    humidity: Humidity,
    #[allow(dead_code)]
    weather_data: Rc<RefCell<WeatherData>>,
}

impl ThirdPartyDisplay {
    #[allow(dead_code)]
    pub fn new(wd: Rc<RefCell<WeatherData>>) -> Rc<RefCell<dyn Observer>> {
        let display = Rc::new(RefCell::new(ThirdPartyDisplay {
            temperature: Temperature(0.0),
            humidity: Humidity(0.0),
            weather_data: wd.clone(),
        }));

        wd.deref().borrow_mut().register_observer(display.clone());

        display
    }
}

impl Observer for ThirdPartyDisplay {
    fn update(&mut self, temp: f32, humidity: f32, _pressure: f32) {
        self.temperature = Temperature((temp - 32.0) / 1.8);
        // self.humidity = Humidity(self.weather_data.deref().borrow().get_humidity());
        self.humidity = Humidity(humidity);

        self.display();
    }
}

impl DisplayElement for ThirdPartyDisplay {
    fn display(&self) {
        let Humidity(hum) = self.humidity;
        let Temperature(temp) = self.temperature;

        println!(
            "Current conditions in Third Party Display: {}C degreses and {}% humidty",
            temp, hum
        );
    }
}

pub struct ForecastDisplay {
    current_pressure: f32,
    last_pressure: f32,
    #[allow(dead_code)]
    weather_data: Rc<RefCell<WeatherData>>,
}

impl ForecastDisplay {
    #[allow(dead_code)]
    pub fn new(wd: Rc<RefCell<WeatherData>>) -> Rc<RefCell<dyn Observer>> {
        let display = Rc::new(RefCell::new(ForecastDisplay {
            current_pressure: 29.92,
            last_pressure: 0.,
            weather_data: wd.clone(),
        }));

        wd.deref().borrow_mut().register_observer(display.clone());

        display
    }
}

impl Observer for ForecastDisplay {
    fn update(&mut self, _temp: f32, _humidity: f32, pressure: f32) {
        self.last_pressure = self.current_pressure;
        self.current_pressure = pressure;

        self.display();
    }
}

impl DisplayElement for ForecastDisplay {
    fn display(&self) {
        print!("Forecast: ");
        let d = &self
            .current_pressure
            .partial_cmp(&self.last_pressure)
            .expect("No ordering possible");

        match d {
            std::cmp::Ordering::Less => println!("Watch out for cooler, rainy weather"),
            std::cmp::Ordering::Equal => println!("More of the same"),
            std::cmp::Ordering::Greater => println!("Improving weather on the way!"),
        }
    }
}
