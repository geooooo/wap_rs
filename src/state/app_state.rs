use rand::{self, Rng};
use super::play_state::PlayState;
use super::track_list_state::TrackListState;
use super::track::Track;
use super::help_target::HelpTarget;

#[derive(Clone)]
pub struct AppState {
    play_state: PlayState,
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
            play_state: PlayState::NoTrack,
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

    pub fn update_tracks(&mut self, tracks: Vec<Track>) {
        self.track_list_state.tracks = tracks;
    }

    pub fn update_track_state(&mut self, track_name: String, is_selected: bool, is_played: bool) {
        if let Some(ref mut played_track) = self.track_list_state.played_track && played_track.name == track_name {
            played_track.is_selected = is_selected;
        }

        let track_from_list = self.track_list_state.tracks.iter_mut().find(|track| track.name == track_name).unwrap();
        track_from_list.is_selected = is_selected;

        if is_played && !track_from_list.is_played {
            track_from_list.is_played = true;
            self.track_list_state.played_track = Some(track_from_list.clone());

            for track in &mut self.track_list_state.tracks {
                if track.name != track_name {
                    track.is_played = false;
                }
            }

            self.set_play_state();

            //TODO:play
        }
    }

    pub fn remove_selected_tracks(&mut self) {
        if let Some(ref played_track) = self.track_list_state.played_track && played_track.is_selected {
            self.track_list_state.played_track = None;
            self.play_state = PlayState::NoTrack;
        }

        self.track_list_state.tracks = self.track_list_state.tracks
            .clone()
            .into_iter()
            .filter(|track| !track.is_selected)
            .collect();
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
                if let Some(ref track) = self.track_list_state.played_track {
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

    pub fn set_play_state(&mut self) {
        self.play_state = if self.track_list_state.has_played_track() {
            PlayState::Play
        } else {
            PlayState::NoTrack
        };
    }

    pub fn toggle_play_state(&mut self) {
        if !self.track_list_state.has_played_track() {
            self.set_next_track();
            self.track_list_state.played_track.as_ref();
        };

        self.play_state = match self.play_state {
            _ if !self.track_list_state.has_played_track() => PlayState::NoTrack,
            PlayState::Play => PlayState::Pause,
            PlayState::NoTrack | PlayState::Pause => PlayState::Play,
        };
    }

    pub fn get_help_text(&self) -> String {
        self.help_text.clone()
    }
    
    pub fn get_play_state(&self) -> PlayState {
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
        match self.track_list_state.played_track {
            Some(ref track) => track.duration,
            None => 0,
        }
    }

    pub fn get_tracks(&self) -> Vec<Track> {
        self.track_list_state.tracks.clone()
    }

    pub fn get_track(&self) -> Option<Track> {
        self.track_list_state.played_track.clone()
    }

    fn get_random_track_index(&self) -> Option<usize> {
        if self.track_list_state.tracks.is_empty() {
            return None;
        }

        let mut rng = rand::thread_rng();
        
        if !self.track_list_state.has_played_track() {
            return Some(rng.gen_range(0..self.track_list_state.tracks.len()));
        }

        if self.track_list_state.tracks.len() == 1 {
            return Some(0);
        }
        
        loop {
            let random_index = rng.gen_range(0..self.track_list_state.tracks.len());
            let random_track = &self.track_list_state.tracks[random_index];
            let played_track = self.track_list_state.played_track.as_ref().unwrap();

            if random_track.name != played_track.name {
                break Some(random_index);
            }
        }
    }

    fn get_next_or_prev_track(&mut self, is_next: bool) -> Option<(&mut Track, Option<usize>)> {
        if !self.track_list_state.has_played_track() {
            let result = 
                if self.track_list_state.tracks.is_empty() {
                    None
                } else if self.is_random {
                    let index = self.get_random_track_index().unwrap();
            
                    Some((&mut self.track_list_state.tracks[index], None))
                } else {
                    let first_track= self.track_list_state.tracks.first_mut().unwrap();

                    Some((first_track, None))
                };

            return result;
        }

        if self.is_loop {
            return Some((self.track_list_state.played_track.as_mut().unwrap(), None));
        }

        let played_index = self.track_list_state.tracks
            .iter()
            .position(|track| self.track_list_state.played_track.as_ref().unwrap().name == track.name)
            .unwrap();

        if self.is_random {
            let random_index = self.get_random_track_index().unwrap();

            return Some((&mut self.track_list_state.tracks[random_index], Some(played_index)));
        } 

        if is_next {
            if played_index == self.track_list_state.tracks.len() - 1 {
                Some((&mut self.track_list_state.tracks[0], Some(played_index)))
            } else {
                Some((&mut self.track_list_state.tracks[played_index + 1], Some(played_index)))
            }
        } else if played_index == 0 {
            Some((self.track_list_state.tracks.last_mut().unwrap(), Some(played_index)))
        } else {
            Some((&mut self.track_list_state.tracks[played_index - 1], Some(played_index)))
        }
    }

    fn set_next_or_prev_track(&mut self, is_next: bool) {
        if self.track_list_state.has_played_track() && self.track_list_state.tracks.len() == 1 {
            return;
        }

        match self.get_next_or_prev_track(is_next) {
            None => (),
            Some((next_track, prev_index)) => {
                next_track.is_played = true;
                self.track_list_state.played_track = Some(next_track.clone());

                if let Some(prev_index) = prev_index {
                    self.track_list_state.tracks[prev_index].is_played = false;
                }
            }
        }
    }

    fn select_or_deselect_all_tracks(&mut self, is_select: bool) {
        for track in &mut self.track_list_state.tracks {
            track.is_selected = is_select;
        }

        if self.track_list_state.has_played_track() {
            self.track_list_state.played_track.as_mut().unwrap().is_selected = is_select;
        }
    }
}