#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum PlayState {
    Play,
    Pause,
    NoTrack,
}