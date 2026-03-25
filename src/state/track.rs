#[derive(Clone)]
pub struct Track {
    pub name: String,
    pub duration: u32,
}

impl Track {
    pub fn new(name: String, duration: u32) -> Self {
        Self {
            name,
            duration,
        }
    }
}