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
}

pub trait Movement {
    fn move_position(self, x: i32, y: i32);
}

pub trait Interaction<T: Survival> {
    fn reproduce(self, other: T);
    fn attack(self, other: T);
    fn eat(self, other: T);
}
