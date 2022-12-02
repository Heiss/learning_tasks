mod display;
mod observer;
mod subject;

use display::*;
use observer::*;
use subject::*;

#[cfg(test)]
mod test_super {
    use std::{cell::RefCell, ops::Deref, rc::Rc};

    use super::*;

    #[test]
    fn test_current_condition_display() {
        let mut _weather_data = Rc::new(RefCell::new(WeatherData::default()));

        let _current_conditions_display = CurrentConditionsDisplay::new(_weather_data.clone());
        let _statistics_display = StatisticsDisplay::new(_weather_data.clone());
        let _forecast_display = ForecastDisplay::new(_weather_data.clone());
        let _third_party_display = ThirdPartyDisplay::new(_weather_data.clone());

        let mut d = _weather_data.deref().borrow_mut();
        d.set_measurements(80.0, 65.0, 30.4);
        d.set_measurements(82.0, 70.0, 29.2);
        d.set_measurements(78.0, 90.0, 29.2);
    }
}
