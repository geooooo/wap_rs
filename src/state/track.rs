#[derive(Debug, Clone)]
pub struct Track {
    pub name: String,
    pub duration: u32,
    pub data: String,
    pub is_selected: bool,
    pub is_played: bool,
}

impl Track {
    pub fn new(name: String, data: String, duration: u32) -> Self {
        Self {
            name,
            duration,
            data,
            is_selected: false,
            is_played: false,
        }
    }
}