pub use askama::Template;

pub trait NonVolatileChunk: askama::Template {
    fn save_to_disk();
}
