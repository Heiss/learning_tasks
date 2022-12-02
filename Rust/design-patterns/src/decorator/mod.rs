mod beverage;
mod condiment;

use beverage::*;
#[allow(unused_imports)]
use condiment::*;

#[cfg(test)]
mod test_super {
    use crate::decorator::{Beverage, DarkRoast, Espresso, HouseBlend, Mocha, Soy, Whip};

    #[test]
    fn test_starbuzzcoffee() {
        let beverage = Espresso::new();
        println!("{} ${}", beverage.get_description(), beverage.cost());

        let beverage2 = DarkRoast::new();
        let beverage2 = Mocha::new(beverage2);
        let beverage2 = Mocha::new(beverage2);
        let beverage2 = Whip::new(beverage2);
        println!("{} ${}", beverage2.get_description(), beverage2.cost());

        let beverage3 = HouseBlend::new();
        let beverage3 = Soy::new(beverage3);
        let beverage3 = Mocha::new(beverage3);
        let beverage3 = Whip::new(beverage3);
        println!("{} ${}", beverage3.get_description(), beverage3.cost());
    }
}
