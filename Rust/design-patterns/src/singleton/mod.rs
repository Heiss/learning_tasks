mod boiler;

#[allow(unused_imports)]
use boiler::ChocolateBoiler;

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_boiler() {
        let boiler = ChocolateBoiler::get_instance();

        {
            let mut boiler = boiler.lock().unwrap();
            boiler.fill();
            boiler.boil();
            boiler.drain();
        }

        let boiler2 = ChocolateBoiler::get_instance();
        {
            let boiler = boiler2.lock().unwrap();
            assert!(boiler.is_empty());
        }

        {
            let mut boiler = boiler.lock().unwrap();
            boiler.fill();
        }

        {
            let boiler = boiler2.lock().unwrap();
            assert!(!boiler.is_empty());
            assert!(!boiler.is_boiled());
        }

        {
            let mut boiler = boiler.lock().unwrap();
            boiler.boil();
        }

        {
            let boiler = boiler2.lock().unwrap();
            assert!(!boiler.is_empty());
            assert!(boiler.is_boiled());
        }

        {
            let mut boiler = boiler.lock().unwrap();
            boiler.drain();
        }

        {
            let boiler = boiler2.lock().unwrap();
            assert!(boiler.is_empty());
        }
    }
}
