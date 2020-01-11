#[derive(Debug)]
pub struct Triple {
    s: Uri,
    p: Uri,
    o: Entity
}

impl Triple {
    pub fn new(s: Uri, p: Uri, o: Entity) -> Triple {
        Triple {
            s, p, o
        }
    }
}

#[derive(Debug)]
pub struct Uri {
    uri: String
}

impl Uri {
    pub fn new(s: &str) -> Uri {
        Uri {
            uri: String::from(s)
        }
    }
}

#[derive(Debug)]
pub enum Entity {
    Uri(Uri),
    Literal(String)
}
