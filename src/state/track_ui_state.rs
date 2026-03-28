#[derive(Clone)]
pub struct TrackUiState {
    pub name: String,
    pub duration: u32,
    pub is_selected: bool,
    pub is_played: bool,
}

impl TrackUiState {
    pub fn new(name: String, duration: u32) -> Self {
        Self {
            name,
            duration,
            is_selected: false,
            is_played: false,
        }
    }

    pub fn id(&self) -> String {
        format!("{}-{}-{}-{}", self.name, self.duration, self.is_selected as u8, self.is_played as u8)
    }
}