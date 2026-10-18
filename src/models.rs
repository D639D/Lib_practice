#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Genre {
    Fantasy,
    ScienceFiction,
    Detective,
    NonFiction,
}
#[derive(Debug)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub year: u32,
    pub genre: Genre,
    pub read: bool,
}

impl Book {
    pub fn collect_as_description(&self) -> String {
        format!("{} - {} - ({})", self.author, self.title, self.year)
    }

    pub fn mark_as_read(&mut self) {
        self.read = true
    }
    pub fn is_old(&self) -> bool {
        self.year < 1950
    }
}
