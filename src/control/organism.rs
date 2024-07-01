use crate::control::position::Position;
use crate::control::world::World;

pub struct Organism {
    sign: String,
    power: i32,
    position: Position,
    initiative: i32,
    power_to_reproduce: i32,
    world: World,
}

impl Organism {
    fn new(
        sign: String,
        power: i32,
        position: Position,
        initiative: i32,
        power_to_reproduce: i32,
        world: World,
    ) -> Organism {
        Organism {
            sign,
            power,
            position,
            initiative,
            power_to_reproduce,
            world,
        }
    }
}
