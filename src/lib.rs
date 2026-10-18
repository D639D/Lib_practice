mod models;
mod storage;

pub use models::{Book, Genre};
pub use storage::{find_oldest, find_read_to_genre};
