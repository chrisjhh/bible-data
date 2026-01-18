use super::book::BibleBook;

/// A struct representing a chapter in the Bible
/// Contains the [BibleBook] and the chapter number
#[allow(dead_code)]
#[derive(Debug)]
pub struct BibleChapter {
    pub book: BibleBook,
    pub chapter: u8,
}

impl BibleChapter {}
