#[derive(Debug)]
pub struct Food {
    pub x: usize,
    pub y: usize,
}

impl Food {
    pub fn new(x: usize, y: usize) -> Food {
        Food { x, y }
    }
}
