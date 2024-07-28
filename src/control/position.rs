#[derive(Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }

    pub fn on_world(&self, world_dim: i32) -> bool {
        self.x >= 0 && self.x < world_dim && self.y >= 0 && self.y < world_dim
    }

    pub fn move_position(&mut self, world_dim: i32, dx: i32, dy: i32) {
        let new_x = self.x + dx;
        let new_y = self.y + dy;

        match new_x > world_dim - 1 {
            true => self.x = world_dim - 1,
            false => match new_x < 0 {
                true => self.x = 0,
                false => self.x = new_x,
            },
        }

        match new_y > world_dim - 1 {
            true => self.y = world_dim - 1,
            false => match new_y < 0 {
                true => self.y = 0,
                false => self.y = new_y,
            },
        }
    }
}
