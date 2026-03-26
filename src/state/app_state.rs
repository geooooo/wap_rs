use super::play_state::PlayState;
use super::track_list_state::TrackListState;
use super::track::Track;

#[derive(Clone)]
pub struct AppState {
    play_state: Option<PlayState>,
    track_list_state: TrackListState,
    help_text: String,
    is_loop: bool,
    is_random: bool,
    time: u32,
    volume: u8,
    speed: u8,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            help_text: String::new(),
            play_state: None,
            track_list_state: TrackListState::default(),
            is_loop: false,
            is_random: false,
            time: 0,
            volume: 100,
            speed: 100,
        }
    }
}

impl AppState {
    pub const MAX_SPEED: u8 = 200;
    pub const MAX_VOLUME: u8 = 100;

    pub fn toggle_track_list_visibility(&mut self) {
        self.track_list_state.is_visible = !self.track_list_state.is_visible;
    }

    pub fn toggle_random(&mut self) {
        self.is_random = !self.is_random;
    }

    pub fn toggle_loop(&mut self) {
        self.is_loop = !self.is_loop;
    }

    // TODO:
    pub fn toggle_play_state(&mut self) {
        self.play_state = match self.play_state {
            None => Some(PlayState::Pause),
            Some(_) => None,
        };
    }

    pub fn get_help_text(&self) -> String {
        self.help_text.clone()
    }
    
    pub fn get_play_state(&self) -> Option<PlayState> {
        self.play_state
    }

    pub fn is_track_list_visible(&self) -> bool {
        self.track_list_state.is_visible
    }

    pub fn get_time(&self) -> u32 {
        self.time
    }

    pub fn get_volume(&self) -> u8 {
        self.volume
    }

    pub fn get_speed(&self) -> u8 {
        self.speed
    }

    pub fn get_track_duration(&self) -> u32 {
        match &self.track_list_state.track {
            Some(track) => track.duration,
            None => 0,
        }
    }

    pub fn get_tracks(&self) -> Vec<Track> {
        self.track_list_state.tracks.clone()
    }
}