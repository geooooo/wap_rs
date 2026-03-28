pub struct TrackFileState {
    pub name: String,
    pub duration: u32,
    pub data: String,
}

impl TrackFileState {
    pub fn new(name: String, data: String, duration: u32) -> Self {
        Self {
            name,
            duration,
            data,
        }
    }
}