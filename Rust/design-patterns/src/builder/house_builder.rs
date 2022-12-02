use super::house::{House, HouseType};

pub trait HouseBuilder {
    fn set_house_type(&mut self, house_type: HouseType) -> &mut Self;
    fn add_walls(&mut self, num: i8, material: &str) -> &mut Self;
    fn add_roof(&mut self, material: &str) -> &mut Self;
    fn add_windows(&mut self, num: i8, material: &str) -> &mut Self;
    fn build(self) -> House;
}

pub struct WoodHouseBuilder {
    house: House,
}

impl WoodHouseBuilder {
    pub fn _new() -> Self {
        Self {
            house: House::default(),
        }
    }
}

impl HouseBuilder for WoodHouseBuilder {
    fn set_house_type(&mut self, house_type: HouseType) -> &mut Self {
        self.house.set_house_type(house_type);
        self
    }

    fn add_walls(&mut self, _num: i8, _material: &str) -> &mut Self {
        todo!()
    }

    fn add_roof(&mut self, _material: &str) -> &mut Self {
        todo!()
    }

    fn add_windows(&mut self, _num: i8, _material: &str) -> &mut Self {
        todo!()
    }

    fn build(self) -> House {
        self.house
    }
}
