#[derive(Debug)]
pub struct Attributes {
    pub power: i32,
    pub health: i32,
    pub attack_damage: i32,
    pub rounds_alive: i32,
    pub sign: char,
}

#[derive(Debug)]
pub struct Reproduction {
    pub no_offspring: i8,
}

// TODO EVOLUTION
#[derive(Debug)]
pub struct Evolution {
    pub chance: i32,
}

#[derive(Debug)]
pub enum Status {
    Alive,
    Dead,
}

pub trait Survival {
    fn live(&mut self);
    fn die(&mut self);
    fn evolve(&mut self);
    fn get_sign(&self) -> char {
        self.get_sign()
    }
    fn get_position(&self) -> (i32, i32);
}

pub trait Movement {
    fn move_position(&mut self, world_dim: i32, x: i32, y: i32);
}

pub trait Interaction {
    fn reproduce(self, other: &dyn Survival);
    fn attack(self, other: &dyn Survival);
    fn eat(self, other: &dyn Survival);
}
