use crate::orgs::orgs_data_model::Survival;

pub struct World {
    x_dim: i32,
    y_dim: i32,
    turn: i32,
    pub organisms: Vec<Box<dyn Survival>>,
}

impl World {
    pub fn new(x_dim: i32, y_dim: i32) -> World {
        World {
            x_dim,
            y_dim,
            turn: 0,
            organisms: Vec::new(),
        }
    }

    pub fn add_organism(&mut self, organism: Box<dyn Survival>) {
        self.organisms.push(organism);
    }

    pub fn dim(&self) -> i32 {
        if self.x_dim > self.y_dim {
            return self.y_dim;
        }
        self.x_dim
    }

    pub fn printwrld(&self) {
        for i in 0..self.dim() {
            for j in 0..self.dim() {
                let mut found = false;
                for org in self.organisms.iter() {
                    if org.get_position().x == i && org.get_position().y == j {
                        print!("{}", org.get_sign());
                        found = true;
                        break;
                    }
                }
                if !found {
                    print!(".");
                }
            }
            println!();
        }
    }

    pub fn make_turn(&mut self) {
        print!("{}[2J", 27 as char);
        for org in self.organisms.iter_mut() {
            org.live();
            self.turn += 1;
        }
        self.printwrld();
    }
}
