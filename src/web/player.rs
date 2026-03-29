use std::sync::Arc;
use std::cell::RefCell;
use futures::future::join_all;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_futures::spawn_local;
use web_sys::{
    ProgressEvent,
    Event,
    File, 
    FileList,
    FileReader,
    HtmlAudioElement,
    AudioContext,
    js_sys::Promise,
};
use crate::state::{TrackUiState, TrackFileState};

pub struct Player {
    audio: Arc<HtmlAudioElement>,
}

impl Player {
    const FFT_SIZE: u32 = 512;

    pub fn new(
        equalizer_level_count: u8,
        volume: u8, 
        speed: u8, 
        on_time_change: impl Fn(u32) + 'static,
        on_equalizer_update: impl Fn(Vec<u8>) + 'static,
    ) -> Self {
        let audio = Arc::new(HtmlAudioElement::new().unwrap());
        audio.set_volume(volume as f64 / 100.0);
        audio.set_playback_rate(speed as f64 / 100.0);

        let audio0 = audio.clone();
        let ontimeupdate = Closure::wrap(Box::new(move |_e: Event| {
            let time = audio0.current_time() as u32;
            on_time_change(time);
        }) as Box<dyn FnMut(Event)>);
    
        audio.set_ontimeupdate(Some(ontimeupdate.as_ref().unchecked_ref()));
        ontimeupdate.forget();

        Self::watch_equalizer_updates(
            equalizer_level_count,
            audio.clone(),
            on_equalizer_update,
        );

        Self {
            audio,
        }
    }

    fn watch_equalizer_updates(
        equalizer_level_count: u8,
        audio: Arc<HtmlAudioElement>,
        on_equalizer_update: impl Fn(Vec<u8>) + 'static,
    ) {
        let audio_context = AudioContext::new().unwrap();
        let audio_source = audio_context.create_media_element_source(&audio).unwrap();
        
        let audio_analyser = Arc::new(audio_context.create_analyser().unwrap());
        audio_analyser.set_fft_size(Self::FFT_SIZE);
        audio_analyser.set_smoothing_time_constant(0.8);

        audio_source.connect_with_audio_node(&audio_analyser).unwrap();
        audio_analyser.connect_with_audio_node(&audio_context.destination()).unwrap();

        let buffer_len = audio_analyser.frequency_bin_count() as usize;
        let mut audio_buffer = Arc::new(vec![0_u8; buffer_len]);

        let onanimationframe: Arc<RefCell<Option<ScopedClosure<'_, dyn FnMut()>>>> = Arc::new(RefCell::new(None));
        let onanimationframe_clone = onanimationframe.clone();

        *onanimationframe_clone.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            if !audio.paused() {
                let _ = audio_context.resume();

                let audio_buffer = Arc::make_mut(&mut audio_buffer);
                audio_analyser.get_byte_frequency_data(audio_buffer);

                let mut euqlizer_levels = vec![0_u8; equalizer_level_count as usize];
                let step_size = buffer_len / equalizer_level_count as usize;
                let mut step = 0_usize;
                
                for i in 0..equalizer_level_count as usize {
                    let mut sum = 0_u64;
                    let start = step;
                    let end = if i == equalizer_level_count as usize - 1 {
                            audio_buffer.len() - step
                        } else {
                            step + step_size
                        };

                    for j in start..end {
                        sum += audio_buffer[j] as u64;
                    }

                    let avg = sum / step_size as u64;

                    euqlizer_levels[i] = avg as u8;
                    step += step_size;
                }

                on_equalizer_update(euqlizer_levels);
            }

            web_sys::window()
                .unwrap()
                .request_animation_frame(
                    onanimationframe.borrow().as_ref().unwrap().as_ref().unchecked_ref()
                )
                .unwrap();
        }) as Box<dyn FnMut()>));

        web_sys::window()
            .unwrap()
            .request_animation_frame(
                onanimationframe_clone.borrow().as_ref().unwrap().as_ref().unchecked_ref()
            )
            .unwrap();
    }

    pub fn set_on_play_end(
        &self, 
        on_play_end: impl Fn() + 'static,
        next_track_getter: impl Fn() -> Arc<String> + 'static,
    ) {
        let onended = Closure::wrap(Box::new(move |_e: Event| {
            on_play_end();
            let data = next_track_getter();
            self.play_track(data);
        }) as Box<dyn FnMut(Event)>);
    
        self.audio.set_onended(Some(onended.as_ref().unchecked_ref()));
        onended.forget();
    }

    pub fn play(&self) {
        let _ = self.audio.play();
    }

    pub fn pause(&self) {
        let _ = self.audio.pause();
    }

    pub fn play_track(&self, data: Arc<String>) {
        let audio = self.audio.clone();
        
        spawn_local(async move {
            audio.set_src(&data);
            audio.load();

            let audio_oncanplay_promise = Promise::new(&mut |resolve, _| {
                let audio0 = audio.clone();

                let oncanplay = Closure::once(
                    move |_: ProgressEvent| {
                        audio0.set_oncanplay(None);
                        resolve.call0(&JsValue::NULL).unwrap();
                    }
                );

                audio.set_oncanplay(Some(oncanplay.as_ref().unchecked_ref()));
                oncanplay.forget();
            });

            JsFuture::from(audio_oncanplay_promise).await.unwrap();
            JsFuture::from(audio.play().unwrap()).await.unwrap();
        });
    }

    pub fn set_volume(&self, volume: u8) {
        self.audio.set_volume(volume as f64 / 100.0);
    }

    pub fn set_speed(&self, speed: u8) {
        self.audio.set_playback_rate(speed as f64 / 100.0);
    }

    pub fn set_time(&self, time: u32) {
        self.audio.set_current_time(time as f64);
    }

    pub fn set_loop(&self, is_loop: bool) {
        self.audio.set_loop(is_loop);
    }

    pub async fn parse_files(&self, files: FileList, current_tracks: Vec<TrackUiState>) -> Vec<TrackFileState> {
        let mut futures = vec![];
        let mut file_names = vec![];

        for i in 0..files.length() {
            let file = files.get(i).unwrap();

            if self.is_track_exists(&file, &current_tracks) {
                continue;
            }

            file_names.push(file.name());
            futures.push(self.get_track_duration(file));
        }

        join_all(futures)
            .await.drain(..)
            .zip(file_names.drain(..))
            .map(|((data, duration), name)| 
                TrackFileState::new(name, data, duration)
            ).collect()
    }

    fn is_track_exists(&self, file: &File, current_tracks: &[TrackUiState]) -> bool {
        current_tracks.iter().find(|track| track.name == file.name()).is_some()
    }

    async fn get_track_duration(&self, file: File) -> (String, u32) {
        let reader = FileReader::new().unwrap();

        let reader_onload_promise = Promise::new(&mut |resolve, _| {
            let onload = Closure::once(
                move |_: ProgressEvent| resolve.call0(&JsValue::NULL).unwrap()
            );

            reader.set_onload(Some(onload.as_ref().unchecked_ref()));
            onload.forget();
        });
        reader.read_as_data_url(&file).unwrap();
        JsFuture::from(reader_onload_promise).await.unwrap();

        let content = reader.result().unwrap().as_string().unwrap();
        let audio = Arc::new(HtmlAudioElement::new_with_src(&content).unwrap());

        let audio_loadedmetadata_promise = Promise::new(&mut |resolve, _| {
            let audio0 = audio.clone();

            let onloadedmetadata = Closure::once(
                move || {
                    audio0.set_onloadedmetadata(None);
                    resolve.call0(&JsValue::NULL).unwrap();
                }
            );

            audio.set_onloadedmetadata(Some(onloadedmetadata.as_ref().unchecked_ref()));
            onloadedmetadata.forget();
        });
        JsFuture::from(audio_loadedmetadata_promise).await.unwrap();

        (content, audio.duration() as u32)
    }
}