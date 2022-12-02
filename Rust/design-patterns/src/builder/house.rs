use std::fmt::{Display, Formatter};

use super::{roof::Roof, wall::Wall, window::Window};

pub enum HouseType {
    Wood,
    _Clay,
    _Gingerbread,
    _Stone,
}

pub struct House {
    name: String,
    house_type: HouseType,
    roof: Roof,
    walls: Vec<Wall>,
    windows: Vec<Window>,
}

#[allow(dead_code)]
impl House {
    pub fn set_house_type(&mut self, house_type: HouseType) -> &mut Self {
        self.house_type = house_type;
        self
    }

    pub fn add_roof(&mut self, roof: Roof) -> &mut Self {
        self.roof = roof;
        self
    }

    pub fn add_wall(&mut self, wall: Wall) -> &mut Self {
        self.walls.push(wall);
        self
    }

    pub fn add_window(&mut self, window: Window) -> &mut Self {
        self.windows.push(window);
        self
    }

    pub fn set_name(&mut self, name: &str) -> &mut Self {
        self.name = name.to_string();
        self
    }
}

impl Default for House {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            house_type: HouseType::Wood,
            roof: Roof::default(),
            walls: Vec::new(),
            windows: Vec::new(),
        }
    }
}

impl Display for House {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut res = format!("---- {} ----\n", self.name);

        for w in &self.windows {
            res = format!("{}\n--- {} ---", res, w.name);
        }

        for w in &self.walls {
            res = format!("{}\n--- {} ---", res, w.name);
        }

        res = format!("{}\n--- {} ---", res, self.roof.name);

        write!(f, "{}", res)
    }
}
