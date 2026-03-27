use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    ProgressEvent,
    File, 
    FileList,
    FileReader,
    HtmlAudioElement,
    AudioContext,
    MediaElementAudioSourceNode,
    AnalyserNode,
    js_sys::Promise,
};
use crate::state::Track;

#[derive(Clone)]
pub struct Player {
    audio: HtmlAudioElement,
    audio_context: AudioContext,
    audio_source: MediaElementAudioSourceNode,
    audio_analyser: AnalyserNode,
    audio_buffer: Vec<u8>,
}

impl Player {
    const FFT_SIZE: u32 = 256;

    pub fn new() -> Self {
        let audio = HtmlAudioElement::new().unwrap();
        let audio_context = AudioContext::new().unwrap();
        let audio_source = audio_context.create_media_element_source(&audio).unwrap();
        
        let audio_analyser = audio_context.create_analyser().unwrap();
        audio_analyser.set_fft_size(Self::FFT_SIZE);

        audio_source.connect_with_audio_node(&audio_analyser).unwrap();
        audio_analyser.connect_with_audio_node(&audio_context.destination()).unwrap();

        let buffer_len = audio_analyser.frequency_bin_count() as usize;
        let audio_buffer = Vec::<u8>::with_capacity(buffer_len);

        Self {
            audio,
            audio_context,
            audio_source,
            audio_analyser,
            audio_buffer,
        }
    }

    pub fn get_audio_buffer(&mut self) -> &[u8] {
        self.audio_analyser.get_byte_frequency_data(&mut self.audio_buffer);

        &self.audio_buffer
    }

    pub async fn parse_files(&self, files: FileList, mut current_tracks: Vec<Track>) -> Vec<Track> {
        let mut new_tracks = Vec::with_capacity(files.length() as usize);

        for i in 0..files.length() {
            let file = files.get(i).unwrap();

            if self.is_track_exists(&file, &current_tracks) {
                continue;
            }

            let name = file.name().clone();
            let (data, duration) = self.get_track_duration(file).await;

            new_tracks.push(Track::new(name, data, duration));
        }

        current_tracks.append(&mut new_tracks);

        current_tracks
    }

    fn is_track_exists(&self, file: &File, current_tracks: &[Track]) -> bool {
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
        let audio = HtmlAudioElement::new_with_src(&content).unwrap();

        let audio_oncanplay_promise = Promise::new(&mut |resolve, _| {
            let oncanplay = Closure::once(
                move || resolve.call0(&JsValue::NULL).unwrap()
            );

            audio.set_oncanplay(Some(oncanplay.as_ref().unchecked_ref()));
            oncanplay.forget();
        });
        JsFuture::from(audio_oncanplay_promise).await.unwrap();

        (content, audio.duration() as u32)
    }
}