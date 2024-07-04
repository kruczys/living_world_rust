#[derive(Debug)]
pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }

    pub fn on_world(&self, world_dim: i32) -> bool {
        self.x >= 0 && self.x < world_dim && self.y >= 0 && self.y < world_dim
    }

    pub fn move_position(&mut self, world_dim: i32, dx: i32, dy: i32) {
        if self.x + dx < world_dim && self.y + dy < world_dim {
            self.x += dx;
            self.y += dy;
        }
    }
}
