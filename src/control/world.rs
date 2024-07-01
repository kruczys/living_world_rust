use crate::control::organism::Organism;

#[derive(Debug)]
pub struct World {
    x_dim: i32,
    y_dim: i32,
    pub organisms: Vec<Organism>,
}

impl World {
    pub fn new(x_dim: i32, y_dim: i32) -> World {
        World {
            x_dim,
            y_dim,
            organisms: Vec::new(),
        }
    }

    pub fn add_organism(&mut self, organism: Organism) {
        self.organisms.push(organism);
    }

    pub fn dim(&self) -> i32 {
        if self.x_dim > self.y_dim {
            return self.y_dim;
        }
        self.x_dim
    }
}
