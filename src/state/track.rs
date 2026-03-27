#[derive(Clone)]
pub struct Track {
    pub name: String,
    pub duration: u32,
    pub is_selected: bool,
    pub is_played: bool,
}

impl Track {
    pub fn new(name: String, duration: u32, s:bool,p:bool) -> Self {
        Self {
            name,
            duration,
            is_selected: s,
            is_played: p,
        }
    }
}