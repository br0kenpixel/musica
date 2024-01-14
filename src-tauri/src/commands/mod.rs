pub type Infallible<T> = Result<T, ()>;
pub type AsyncVoid = Infallible<()>;

pub mod dirs;
pub mod library;
pub mod playback;
pub mod settings;
