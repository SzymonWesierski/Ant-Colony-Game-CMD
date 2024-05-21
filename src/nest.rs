#[derive(Debug)]
pub struct Nest {
    pub x: usize,
    pub y: usize,
}

impl Nest {
    pub fn new(x: usize, y: usize) -> Nest {
        Nest { x, y }
    }
}
