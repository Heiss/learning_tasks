use std::{cell::RefCell, ops::Deref, rc::Rc};

use crate::observer::Observer;

pub trait Subject {
    fn register_observer(&mut self, o: Rc<RefCell<dyn Observer>>);
    fn remove_observer(&mut self, o: Rc<RefCell<dyn Observer>>);
    fn notify_observers(&mut self);
}

#[derive(Default)]
pub struct WeatherData {
    observers: Vec<Rc<RefCell<dyn Observer>>>,
    temperature: f32,
    humidity: f32,
    pressure: f32,
}

#[allow(dead_code)]
impl WeatherData {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn get_temperature(&self) -> f32 {
        self.temperature
    }
    pub fn get_humidity(&self) -> f32 {
        self.humidity
    }
    pub fn get_pressure(&self) -> f32 {
        self.pressure
    }
    fn measurements_changed(&mut self) {
        self.notify_observers();
    }

    pub fn set_measurements(&mut self, temp: f32, humidity: f32, pressure: f32) {
        self.temperature = temp;
        self.humidity = humidity;
        self.pressure = pressure;
        self.measurements_changed();
    }
}

impl Subject for WeatherData {
    fn register_observer(&mut self, o: Rc<RefCell<dyn Observer>>) {
        self.observers.push(o);
    }

    fn remove_observer(&mut self, o: Rc<RefCell<dyn Observer>>) {
        self.observers.swap_remove(
            self.observers
                .iter()
                .position(|x| std::cmp::PartialEq::eq(x, &o))
                .expect(""),
        );
    }

    fn notify_observers(&mut self) {
        self.observers.iter().for_each(|o| {
            o.deref()
                .borrow_mut()
                .update(self.temperature, self.humidity, self.pressure);
        })
    }
}
