use std::sync::Arc;
use futures::future::join_all;
use leptos::leptos_dom::logging::console_log;
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
    MediaElementAudioSourceNode,
    AnalyserNode,
    js_sys::Promise,
};
use crate::state::{TrackUiState, TrackFileState};

#[derive(Clone)]
pub struct Player {
    audio: Arc<HtmlAudioElement>,
    _audio_context: AudioContext,
    _audio_source: MediaElementAudioSourceNode,
    _audio_analyser: AnalyserNode,
    _audio_buffer: Vec<u8>,
}

impl Player {
    const FFT_SIZE: u32 = 256;

    pub fn new(volume: u8, speed: u8, on_time_change: impl Fn(u32) + 'static) -> Self {
        let audio = Arc::new(HtmlAudioElement::new().unwrap());
        audio.set_volume(volume as f64 / 100.0);
        audio.set_playback_rate(speed as f64 / 100.0);
        audio.set_current_time(0.0);
        audio.set_preload("metadata");
        audio.set_autoplay(false);
        
        let audio0 = audio.clone();
        let ontimeupdate = Closure::wrap(Box::new(move |_e: Event| {
            let time = audio0.current_time() as u32;
            on_time_change(time);
        }) as Box<dyn FnMut(Event)>);
    
        audio.set_ontimeupdate(Some(ontimeupdate.as_ref().unchecked_ref()));
        ontimeupdate.forget();

        let _audio_context = AudioContext::new().unwrap();
        let _audio_source = _audio_context.create_media_element_source(&audio).unwrap();
        
        let _audio_analyser = _audio_context.create_analyser().unwrap();
        _audio_analyser.set_fft_size(Self::FFT_SIZE);

        _audio_source.connect_with_audio_node(&_audio_analyser).unwrap();
        _audio_analyser.connect_with_audio_node(&_audio_context.destination()).unwrap();

        let buffer_len = _audio_analyser.frequency_bin_count() as usize;
        let _audio_buffer = Vec::<u8>::with_capacity(buffer_len);

        Self {
            audio,
            _audio_context,
            _audio_source,
            _audio_analyser,
            _audio_buffer,
        }
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
            audio.pause().unwrap();
            audio.set_current_time(0.0);

            audio.set_src(&data);
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

    pub fn _get_audio_buffer(&mut self) -> &[u8] {
        self._audio_analyser.get_byte_frequency_data(&mut self._audio_buffer);

        &self._audio_buffer
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

    pub async fn get_track_duration(&self, file: File) -> (String, u32) {
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