use crate::control::position::Position;
use crate::orgs::orgs_data_model::Attributes;
use crate::orgs::orgs_data_model::Evolution;
use crate::orgs::orgs_data_model::Movement;
use crate::orgs::orgs_data_model::Reproduction;
use crate::orgs::orgs_data_model::Status;
use crate::orgs::orgs_data_model::Survival;

#[derive(Debug)]
pub struct SimpleOrganism {
    attr: Attributes,
    repr: Reproduction,
    evo: Evolution,
    pos: Position,
    status: Status,
}

impl SimpleOrganism {
    pub fn new(at_x: i32, at_y: i32) -> Self {
        Self {
            attr: Attributes {
                power: 10,
                health: 10,
                attack_damage: 10,
                sign: 'S',
                rounds_alive: 0,
            },
            repr: Reproduction { no_offspring: 2 },
            evo: Evolution { chance: 10 },
            pos: Position::new(at_x, at_y),
            status: Status::Alive,
        }
    }
}

impl Survival for SimpleOrganism {
    fn live(&mut self) {
        self.attr.power += 2;
        self.attr.rounds_alive += 1;
    }

    fn die(&mut self) {
        self.status = Status::Dead;
    }
    // TODO: Implement evolution
    fn evolve(&mut self) {
        println!("I am evolving");
    }

    fn get_position(&self) -> &Position {
        &self.pos
    }

    fn get_sign(&self) -> char {
        self.attr.sign
    }
}

impl Movement for SimpleOrganism {
    fn move_position(&mut self, world_dim: i32, x: i32, y: i32) {
        self.pos.move_position(world_dim, x, y);
    }
}
