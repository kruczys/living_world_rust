use crate::control::organism::Organism;

pub struct World {
    x_dim: i32,
    y_dim: i32,
    organisms: Vec<Organism>,
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
}
