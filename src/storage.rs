use crate::models::{Book, Genre};

pub fn find_oldest(books: &[Book]) -> Option<&Book> {
    if books.is_empty() {
        None
    } else {
        let mut tmp = &books[0];
        for item in books {
            if item.year < tmp.year {
                tmp = item;
            }
        }
        Some(tmp)
    }
}

pub fn find_read_to_genre(books: &[Book], style: Genre) -> Option<&Book> {
    for book in books {
        if book.genre == style && !book.read {
            return Some(book)
        }
    }
    None
}