use crate::triple::Triple;

#[derive(Debug)]
pub struct Graph {
    g: Vec<Triple>
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            g: Vec::new()
        }
    }

    pub fn add(&mut self, triple: Triple) {
        self.g.push(triple);
    }
}
