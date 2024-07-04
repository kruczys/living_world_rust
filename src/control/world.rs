// use crate::control::organism::Exist;
use crate::control::orgs_data_model::Organism;

#[derive(Debug)]
pub struct World<'a> {
    x_dim: i32,
    y_dim: i32,
    pub organisms: Vec<&'a Organism>,
}

impl<'a> World<'a> {
    pub fn new(x_dim: i32, y_dim: i32) -> World<'a> {
        World {
            x_dim,
            y_dim,
            organisms: Vec::new(),
        }
    }

    pub fn add_organism(&mut self, organism: &'a Organism) {
        self.organisms.push(organism);
    }

    pub fn dim(&self) -> i32 {
        if self.x_dim > self.y_dim {
            return self.y_dim;
        }
        self.x_dim
    }
}
