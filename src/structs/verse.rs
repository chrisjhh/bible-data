use std::fmt::Display;
use std::str::FromStr;

use crate::structs::errors::{NoChapterSpecified, NoSuchBookError};

use super::book::BibleBook;
use super::chapterandverseorverse::ChapterAndVerseOrVerse;
use super::errors::ParseError;

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

#[allow(dead_code)]
impl BibleVerse {
    pub fn parse(text: &str) -> Option<Self> {
        text.parse().ok()
    }

    pub fn new(book: BibleBook, chapter: u8, verse: u8) -> Self {
        BibleVerse {
            book,
            chapter,
            verse,
        }
    }
}

impl TryFrom<&str> for BibleVerse {
    type Error = ParseError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl FromStr for BibleVerse {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Start by attempting to parse the book from the abbrev at the start of the text
        // as this is very quick
        let book = BibleBook::parse_abbrev(s)
            .ok_or_else(|| NoSuchBookError::new("No matching abbreviation".to_string()))?;
        // Result of parse_book_abbrev ends with end of string or space character
        // We can find rest of string (if any) by looking for the first space character
        match s.find(" ") {
            None => Err(NoChapterSpecified::new("No chapter/verse specified.".to_string()).into()), // There is no chapter/verse specified
            Some(pos) => {
                let remain = &s[pos + 1..];
                match ChapterAndVerseOrVerse::from_str(remain)? {
                    ChapterAndVerseOrVerse::JustVerse(verse) => {
                        // No chapter
                        // This is invalid, unless the book only has one chapter
                        // In which case, chapter one is implicit
                        match book.number_of_chapters() {
                            1 => Ok(BibleVerse {
                                book,
                                chapter: 1,
                                verse,
                            }),
                            _ => Err(NoChapterSpecified::new(
                                "Chapter can only be ommited for single-chapter books".to_string(),
                            )
                            .into()),
                        }
                    }
                    ChapterAndVerseOrVerse::Both(cv) => Ok(BibleVerse {
                        book,
                        chapter: cv.chapter,
                        verse: cv.verse,
                    }),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            BibleVerse::parse("Ge 1:1").unwrap(),
            BibleVerse {
                book: BibleBook::Genesis,
                chapter: 1,
                verse: 1
            }
        );
        assert_eq!(
            BibleVerse::parse("Ps 119:176").unwrap(),
            BibleVerse {
                book: BibleBook::Psalms,
                chapter: 119,
                verse: 176
            }
        );
        // First chapter is implicit for single-chapter books
        assert_eq!(
            BibleVerse::parse("Jude 5").unwrap(),
            BibleVerse {
                book: BibleBook::Jude,
                chapter: 1,
                verse: 5
            }
        );
        // But it may also be specified implicitly
        assert_eq!(
            BibleVerse::parse("Jude 1:5").unwrap(),
            BibleVerse {
                book: BibleBook::Jude,
                chapter: 1,
                verse: 5
            }
        );
        // Should not be implicit for other books
        assert_eq!(BibleVerse::parse("Judges 5"), None);
    }

    #[test]
    fn test_display() {
        assert_eq!(
            format!(
                "{}",
                BibleVerse {
                    book: BibleBook::Acts,
                    chapter: 2,
                    verse: 1
                }
            ),
            "Ac 2:1"
        );
    }

    #[test]
    fn test_ord() {
        let v1 = BibleVerse {
            book: BibleBook::Genesis,
            chapter: 1,
            verse: 1,
        };
        let v2 = BibleVerse {
            book: BibleBook::Genesis,
            chapter: 1,
            verse: 10,
        };
        let v3 = BibleVerse {
            book: BibleBook::Genesis,
            chapter: 2,
            verse: 1,
        };
        let v4 = BibleVerse {
            book: BibleBook::Exodus,
            chapter: 1,
            verse: 1,
        };
        let v5 = BibleVerse {
            book: BibleBook::Genesis,
            chapter: 1,
            verse: 1,
        };
        assert!(v1 < v2);
        assert!(v3 > v2);
        assert!(v3 > v1);
        assert!(v4 > v3);
        assert!(v1 < v4);
        assert!(v1 == v5);
    }
}
