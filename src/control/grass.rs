use crate::control::orgs_data_model::Attributes;
use crate::control::orgs_data_model::Evolution;
use crate::control::orgs_data_model::Reproduction;
use crate::control::orgs_data_model::Status;
use crate::control::orgs_data_model::Survival;
use crate::control::position::Position;

#[derive(Debug)]
pub struct Grass {
    attr: Attributes,
    repr: Reproduction,
    evo: Evolution,
    pos: Position,
    status: Status,
}

impl Grass {
    pub fn new(at_x: i32, at_y: i32) -> Self {
        Self {
            attr: Attributes {
                power: 10,
                health: 10,
                attack_damage: 10,
                sign: 'G',
                rounds_alive: 0,
            },
            repr: Reproduction { no_offspring: 2 },
            evo: Evolution { chance: 10 },
            pos: Position::new(at_x, at_y),
            status: Status::Alive,
        }
    }
}

impl Survival for Grass {
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