use super::track::Track;

#[derive(Clone)]
pub struct TrackListState {
    pub is_visible: bool,
    pub played_track: Option<Track>,
    pub tracks: Vec<Track>, 
}

impl Default for TrackListState {
    fn default() -> Self {
        Self {
            is_visible: true,
            played_track: None,
            tracks: vec![
                Track::new("Track0".to_string(), 1234, false, false),
                Track::new("Track2".to_string(), 4321, false, false),
                Track::new("Track3".to_string(), 460, false, false),
                Track::new("Track4".to_string(), 4, false, false),
                Track::new("Track5".to_string(), 124, false, false),
                Track::new("Track1".to_string(), 34, false, false),
            ],
        }
    }
}

impl TrackListState {
    pub fn has_played_track(&self) -> bool {
        self.played_track.is_some()
    }
}