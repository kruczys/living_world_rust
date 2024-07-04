// use crate::control::organism::Exist;

#[derive(Debug)]
pub struct World<'a, T> {
    x_dim: i32,
    y_dim: i32,
    pub organisms: Vec<&'a T>,
}

impl<'a, T> World<'a, T> {
    pub fn new(x_dim: i32, y_dim: i32) -> World<'a, T> {
        World {
            x_dim,
            y_dim,
            organisms: Vec::new(),
        }
    }

    pub fn add_organism(&mut self, organism: &'a T) {
        self.organisms.push(organism);
    }

    pub fn dim(&self) -> i32 {
        if self.x_dim > self.y_dim {
            return self.y_dim;
        }
        self.x_dim
    }
}
