use std::fmt::Display;

use super::book::BibleBook;
use super::chapterandverse::ChapterAndVerse;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
pub struct BibleVerse {
    pub book: BibleBook,
    pub chapter: u8,
    pub verse: u8,
}

impl Display for BibleVerse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}:{}", self.book.abbrev(), self.chapter, self.verse)
    }
}

impl PartialOrd for BibleVerse {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BibleVerse {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.book.cmp(&other.book) {
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Equal => match self.chapter.cmp(&other.chapter) {
                std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
                std::cmp::Ordering::Less => std::cmp::Ordering::Less,
                std::cmp::Ordering::Equal => self.verse.cmp(&other.verse),
            },
        }
    }
}

/*
#[allow(dead_code)]
impl BibleVerse {
    pub fn parse(text: &str) -> Option<Self> {
        // Start by attempting to parse the book from the abbrev at the start of the text
        // as this is very quick
        match parse_book_abbrev(text) {
            None => None,
            Some(index) => {
                let book = BibleBook::from_index(index)
                    .expect("Result of parse_book_abbrev should be in range");
                // Result of parse_book_abbrev ends with end of string or space character
                // We can find rest of strin (if any) by looking for the first space character
                let mut chapter: Option<u8> = None;
                let mut verse: Option<u8> = None;
                match text.find(" ") {
                    None => None, // There is no chapter/verse specified
                    Some(pos) => {
                        let remain = &text[pos + 1..];
                        match ChapterAndVerse::parse(remain) {
                            None => {
                                // No chapter and verse specified
                                // This is invalid, unless the book only has one chapter
                                // In which case, chapter one is implicit
                                match book.number_of_chapters() {
                                    1 => Some(BibleChapter { book, chapter: 1 }),
                                    _ => None,
                                }
                            }
                        // This should be the chapter number
                        match u8::from_str(remain) {
                            Err(_) => None,
                            Ok(chapter) if chapter as u32 > book.number_of_chapters() => None,
                            Ok(chapter) => {
                                // We have a valid reference!
                                Some(BibleChapter { book, chapter })
                            }
                        }
                    }
                }
            }
        }
    }
}
*/
