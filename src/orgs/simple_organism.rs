use crate::control::position::Position;
use crate::orgs::orgs_data_model::Attributes;
use crate::orgs::orgs_data_model::Evolution;
use crate::orgs::orgs_data_model::Movement;
use crate::orgs::orgs_data_model::Reproduction;
use crate::orgs::orgs_data_model::Status;
use crate::orgs::orgs_data_model::Survival;
use rand::Rng;

pub struct SimpleOrganism {
    attr: Attributes,
    repr: Reproduction,
    evo: Evolution,
    pos: Position,
    status: Status,
}

impl SimpleOrganism {
    pub fn new(at_x: i32, at_y: i32, world_dim: i32) -> Self {
        Self {
            attr: Attributes {
                power: 10,
                health: 10,
                attack_damage: 10,
                sign: 'S',
                rounds_alive: 0,
                world_dim: world_dim,
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
        self.attr.health -= 1;
        if self.attr.health == 0 {
            self.die()
        }
        self.attr.rounds_alive += 1;
        self.move_position()
    }

    fn die(&mut self) {
        self.status = Status::Dead;
        self.attr.sign = 'X';
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

    fn get_status(&self) -> &Status {
        &self.status
    }
}

impl Movement for SimpleOrganism {
    fn move_position(&mut self) {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-1..2);
        let y = rng.gen_range(-1..2);
        self.pos.move_position(self.attr.world_dim, x, y);
    }
}
