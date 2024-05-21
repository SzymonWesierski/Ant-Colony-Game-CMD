use rand::Rng;

#[derive(Debug)]
pub struct Ant {
    pub x: usize,
    pub y: usize,
    pub carrying_food: bool,
    pub moves: Vec<(usize, usize)>,
}

impl Ant {
    pub fn new(x: usize, y: usize) -> Ant {
        let mut moves = Vec::new();
        moves.push((x, y));

        Ant {
            x,
            y,
            carrying_food: false,
            moves,
        }
    }

    pub fn move_ant(&mut self, world_size: (usize, usize)) -> (usize, usize) {
        if self.carrying_food && !self.moves.is_empty() {
            let (new_x, new_y) = self.moves.pop().unwrap_or((self.x, self.y));
            self.x = new_x;
            self.y = new_y;
        } else {
            let mut rng = rand::thread_rng();
            let dx = rng.gen_range(0..3) as isize - 1;
            let dy = rng.gen_range(0..3) as isize - 1;

            let new_x = (self.x as isize + dx).clamp(0, (world_size.0 - 1) as isize) as usize;
            let new_y = (self.y as isize + dy).clamp(0, (world_size.1 - 1) as isize) as usize;

            self.moves.push((new_x, new_y));
            self.x = new_x;
            self.y = new_y;
        }
        (self.x, self.y)
    }
}