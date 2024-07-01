use crate::control::position::Position;

#[derive(Debug)]
pub struct Organism {
    sign: String,
    power: i32,
    pub position: Position,
    initiative: i32,
    power_to_reproduce: i32,
}

impl Organism {
    pub fn origin() -> Organism {
        Organism {
            sign: String::from("O"),
            power: 10,
            position: Position::new(0, 0),
            initiative: 0,
            power_to_reproduce: 10,
        }
    }

    pub fn new(
        sign: String,
        power: i32,
        position: Position,
        initiative: i32,
        power_to_reproduce: i32,
    ) -> Organism {
        Organism {
            sign,
            power,
            position,
            initiative,
            power_to_reproduce,
        }
    }

    pub fn power(&self) -> i32 {
        self.power
    }
}
