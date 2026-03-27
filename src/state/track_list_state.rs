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
            tracks: vec![],
        }
    }
}

impl TrackListState {
    pub fn has_played_track(&self) -> bool {
        self.played_track.is_some()
    }
}