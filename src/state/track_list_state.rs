use super::track::Track;

#[derive(Clone)]
pub struct TrackListState {
    pub is_visible: bool,
    pub track: Option<Track>,
    pub tracks: Vec<Track>, 
}

impl Default for TrackListState {
    fn default() -> Self {
        Self {
            is_visible: true,
            track: None,
            tracks: vec![],
        }
    }
}