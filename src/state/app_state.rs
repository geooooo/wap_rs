use std::sync::Arc;
use rand::{self, Rng};
use super::play_state::PlayState;
use super::track_file_state::TrackFileState;
use super::track_ui_state::TrackUiState;
use super::help_target::HelpTarget;

#[derive(Clone)]
pub struct AppState {
    play_state: PlayState,
    track_index: Option<usize>,
    ui_tracks: Vec<TrackUiState>, 
    tracks_data: Vec<Arc<String>>,
    equalizer_levels: Vec<u8>,
    help_text: String,
    is_track_list_visible: bool,
    is_loop: bool,
    is_random: bool,
    is_time_hovered: bool,
    time: u32,
    volume: u8,
    speed: u8,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            play_state: PlayState::NoTrack,
            track_index: None,
            ui_tracks: vec![],
            tracks_data: vec![],
            equalizer_levels: vec![0; Self::EQUALIZER_LEVEL_COUNT as usize],
            help_text: String::new(),
            is_track_list_visible: true,
            is_loop: false,
            is_random: false,
            is_time_hovered: false,
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
    pub const EQUALIZER_LEVEL_COUNT: u8 = 30;

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

    pub fn update_tracks(&mut self, new_tracks: Vec<TrackFileState>) {
        new_tracks.into_iter().for_each(|new_track| {
            self.ui_tracks.push(TrackUiState::new(new_track.name, new_track.duration));
            self.tracks_data.push(Arc::new(new_track.data));
        });
    }

    pub fn update_track_state(&mut self, track_name: String, is_selected: bool, is_played: bool) {
        if let Some(index) = self.track_index && self.ui_tracks[index].name == track_name {
            return;
        }

        let (index_from_list, track_from_list) = self.ui_tracks
            .iter_mut()
            .enumerate()
            .find(|(_, track)| track.name == track_name).unwrap();
        track_from_list.is_selected = is_selected;

        if is_played && !track_from_list.is_played {
            track_from_list.is_played = true;

            if let Some(index) = self.track_index {
                self.ui_tracks[index].is_played = false;
            }
            
            self.track_index = Some(index_from_list);

            self.set_play_state();
        }
    }

    pub fn remove_selected_tracks(&mut self) {
        if let Some(index) = self.track_index && self.ui_tracks[index].is_selected {
            self.track_index = None;
            self.play_state = PlayState::NoTrack;
            self.time = 0;

            self.remove_and_reallocate_selected_tracks();
        } else if let Some(index) = self.track_index {
            let played_track_name = self.ui_tracks[index].name.clone();

            self.remove_and_reallocate_selected_tracks();

            self.track_index = self.ui_tracks
                .iter()
                .position(|track| track.name == played_track_name);
        } else {
            self.remove_and_reallocate_selected_tracks();
        }
    }

    pub fn select_all_tracks(&mut self) {
        self.select_or_deselect_all_tracks(true);
    }

    pub fn deselect_all_tracks(&mut self) {
        self.select_or_deselect_all_tracks(false);
    }

    pub fn set_next_track(&mut self) {
        self.set_next_or_prev_track(true);
    }

    pub fn set_prev_track(&mut self) {
        self.set_next_or_prev_track(false);
    }

    pub fn set_time(&mut self, time: u32) {
        self.time = time;

        self.set_help_text(HelpTarget::TimeLine);
    }

    pub fn set_equalizer_levels(&mut self, equalizer_levels: Vec<u8>) {
        self.equalizer_levels = equalizer_levels;

    }

    pub fn clear_equalizer_levels(&mut self) {
        self.equalizer_levels = vec![0_u8; Self::EQUALIZER_LEVEL_COUNT as usize];

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
            HelpTarget::TimeLine if self.is_time_hovered => self.help_text = 
                if let Some(index) = self.track_index {
                    format!(
                        "{} / {}", 
                        AppState::format_time(self.time), 
                        AppState::format_time(self.ui_tracks[index].duration),
                    )
                } else {
                    "-- / --".to_string()
                },
            HelpTarget::ListButton => self.help_text =
                if self.is_track_list_visible {
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
            HelpTarget::SpeedLine => self.help_text = format!("Speed: {}%", self.speed),
            HelpTarget::VolumeLine => self.help_text = format!("Volume: {}%", self.volume),
            _ => (),
        };
    }

    pub fn toggle_track_list_visibility(&mut self) {
        self.is_track_list_visible = !self.is_track_list_visible;

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

    pub fn set_is_time_hovered(&mut self, is_time_hovered: bool) {
        self.is_time_hovered = is_time_hovered;
    }

    pub fn set_play_state(&mut self) {
        self.play_state = if self.track_index.is_some() {
            PlayState::Play
        } else {
            PlayState::NoTrack
        };
    }

    pub fn toggle_play_state(&mut self) -> (PlayState, PlayState) {
        let prev_play_state = self.play_state;

        if self.track_index.is_none() {
            self.set_next_track();
        };

        self.play_state = match self.play_state {
            _ if self.track_index.is_none() => PlayState::NoTrack,
            PlayState::Play => PlayState::Pause,
            PlayState::NoTrack | PlayState::Pause => PlayState::Play,
        };

        (prev_play_state, self.play_state)
    }

    pub fn get_help_text(&self) -> String {
        self.help_text.clone()
    }
    
    pub fn get_play_state(&self) -> PlayState {
        self.play_state
    }

    pub fn is_track_list_visible(&self) -> bool {
        self.is_track_list_visible
    }

    pub fn is_loop(&self) -> bool {
        self.is_loop
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

    pub fn get_equalizer_levels(&self) -> Vec<u8> {
        self.equalizer_levels.clone()
    }

    pub fn get_track_duration(&self) -> u32 {
        match self.track_index {
            Some(index) => self.ui_tracks[index].duration,
            None => 0,
        }
    }

    pub fn get_tracks(&self) -> Vec<TrackUiState> {
        self.ui_tracks.clone()
    }

    pub fn get_track(&self) -> Option<Arc<String>> {
        match self.track_index {
            None => None,
            Some(index) => Some(self.tracks_data[index].clone()),
        }
    }

    fn get_random_track_index(&mut self) -> Option<usize> {
        if self.ui_tracks.is_empty() {
            return None;
        }
        
        let mut rng = rand::thread_rng();

        if self.track_index.is_none() {
            return Some(rng.gen_range(0..self.ui_tracks.len()));
        }

        if self.ui_tracks.len() == 1 {
            return Some(0);
        }
        
        loop {
            let random_index = rng.gen_range(0..self.ui_tracks.len());
            let random_track = &self.ui_tracks[random_index];
            let played_track = &self.ui_tracks[self.track_index.unwrap()];

            if random_track.name != played_track.name {
                break Some(random_index);
            }
        }
    }

    fn get_next_or_prev_track_index(&mut self, is_next: bool) -> Option<usize> {
        if self.track_index.is_none() {
            return if self.ui_tracks.is_empty() {
                None
            } else if self.is_random {
                self.get_random_track_index()
            } else {
                Some(0)
            };
        }
        
        let track_index = self.track_index.unwrap();

        if self.is_loop {
            return Some(track_index);
        }

        if self.is_random {
            return self.get_random_track_index();
        } 

        if is_next {
            if track_index == self.ui_tracks.len() - 1 {
                Some(0)
            } else {
                Some(track_index + 1)
            }
        } else if track_index == 0 {
            Some(self.ui_tracks.len() - 1)
        } else {
            Some(track_index - 1)
        }
    }

    fn set_next_or_prev_track(&mut self, is_next: bool) {
        if self.track_index.is_some() && self.ui_tracks.len() == 1 {
            return;
        }

        match self.get_next_or_prev_track_index(is_next) {
            None => (),
            Some(new_track_index) => {
                if let Some(track_index) = self.track_index && track_index != new_track_index {
                    self.ui_tracks[track_index].is_played = false;
                }

                self.ui_tracks[new_track_index].is_played = true;
                self.track_index = Some(new_track_index);
            }
        }
    }

    fn select_or_deselect_all_tracks(&mut self, is_select: bool) {
        self.ui_tracks.iter_mut().for_each(|track| {
            if !track.is_played {
                track.is_selected = is_select;
            }
        });
    }

    fn remove_and_reallocate_selected_tracks(&mut self) {
        let mut new_ui_tracks: Vec<TrackUiState> = vec![];
        let mut tracks_data: Vec<Arc<String>> = vec![];

        self.ui_tracks
            .drain(..)
            .zip(
                self.tracks_data.drain(..)
                .map(|track_arc| Arc::try_unwrap(track_arc).unwrap())
            )
            .for_each(|(track, data)| {
                if !track.is_selected {
                    new_ui_tracks.push(track);
                    tracks_data.push(Arc::new(data));
                }
            });
        
        self.ui_tracks = new_ui_tracks;
        self.tracks_data = tracks_data;
    }
}