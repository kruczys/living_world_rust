use crate::control::position::Position;

pub struct Grass;
pub struct Sheep;

enum Organism {
    Grass(Grass),
    Sheep(Sheep),
}

pub trait Exist<T> {
    fn live(self);
    fn die(self);
    fn evolve(self);
    fn can_act(self, other: T) -> bool;
}

pub trait Reproduce<T> {
    fn reproduce(self, other: T);
}
