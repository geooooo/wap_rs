use super::play_state::PlayState;
use super::track_list_state::TrackListState;
use super::track::Track;
use super::help_target::HelpTarget;

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
    pub const VOLUME_STEP: u8 = 10;

    pub fn format_time(time: u32) -> String {
        let hours = time / 3600;
        let minutes = time % 3600 / 60;
        let seconds = time % 60;

        if hours > 0 {
            format!("{hours:0>2}:{minutes:0>2}:{seconds:0>2}")
        } else {
            format!("{minutes:0>2}:{seconds:0>2}")
        }
    }

    pub fn set_current_time(&mut self, time: u32) {
        self.time = time;

        self.set_help_text(HelpTarget::TimeLine);
    }

    pub fn set_speed(&mut self, speed: u8) {
        self.speed = speed;

        self.set_help_text(HelpTarget::SpeedLine);
    }

    pub fn inc_volume(&mut self) {
        if self.volume == Self::MAX_VOLUME {
            return;
        }

        let m = self.volume % Self::VOLUME_STEP;
        self.volume += if m == 0 { Self::VOLUME_STEP } else { Self::VOLUME_STEP - m };
    }

    pub fn dec_volume(&mut self) {
        if self.volume == 0 {
            return;
        }

        let m = self.volume % Self::VOLUME_STEP;
        self.volume -= if m == 0 { Self::VOLUME_STEP } else { m };
    }

    pub fn set_volume(&mut self, volume: u8) {
        self.volume = volume;

        self.set_help_text(HelpTarget::VolumeLine);
    }

    pub fn set_help_text(&mut self, target: HelpTarget) {
        match target {
            HelpTarget::ListButton => self.help_text =
                if self.track_list_state.is_visible {
                    "Hide track list".to_string()
                } else {
                    "Show track list".to_string()
                },
            HelpTarget::RandomButton => self.help_text = 
                if self.is_random {
                    "Off randomizing tracks".to_string()
                } else {
                    "On randomizing tracks".to_string()
                },
            HelpTarget::LoopButton => self.help_text = 
                if self.is_loop {
                    "Off cicle repeat for played track".to_string()
                } else {
                    "On cicle repeat for played track".to_string()
                },
            HelpTarget::TimeLine => self.help_text = 
                if let Some(ref track) = self.track_list_state.track {
                    format!(
                        "{} / {}", 
                        AppState::format_time(self.time), 
                        AppState::format_time(track.duration),
                    )
                } else {
                    format!("-- / --")
                },
            HelpTarget::SpeedLine => self.help_text = format!("Speed: {}%", self.speed),
            HelpTarget::VolumeLine => self.help_text = format!("Volume: {}%", self.volume),
        };
    }

    pub fn toggle_track_list_visibility(&mut self) {
        self.track_list_state.is_visible = !self.track_list_state.is_visible;

        self.set_help_text(HelpTarget::ListButton);
    }

    pub fn toggle_random(&mut self) {
        self.is_random = !self.is_random;

        self.set_help_text(HelpTarget::RandomButton);
    }

    pub fn toggle_loop(&mut self) {
        self.is_loop = !self.is_loop;

        self.set_help_text(HelpTarget::LoopButton);
    }

    pub fn toggle_play_state(&mut self) {
        self.play_state = match self.play_state {
            None if self.track_list_state.track.is_none() => None,
            None => Some(PlayState::Pause),
            Some(PlayState::Play) => Some(PlayState::Pause),
            Some(PlayState::Pause) => Some(PlayState::Play),
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
        match self.track_list_state.track {
            Some(ref track) => track.duration,
            None => 0,
        }
    }

    pub fn get_tracks(&self) -> Vec<Track> {
        self.track_list_state.tracks.clone()
    }
}