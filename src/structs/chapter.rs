use super::book::BibleBook;
use super::errors::{
    ChapterOutOfRange, NoChapterSpecified, NoSuchBookError, NotANumber, ParseChapterError,
};
use crate::parse_book_abbrev;
use std::{fmt::Display, str::FromStr};

/// A struct representing a chapter in the Bible
/// Contains the [BibleBook] and the chapter number
#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
pub struct BibleChapter {
    pub book: BibleBook,
    pub chapter: u8,
}

impl Display for BibleChapter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.book.abbrev(), self.chapter)
    }
}

#[allow(dead_code)]
impl BibleChapter {
    /// Construct a new BibleChapter from the book and chapter
    pub fn new(book: BibleBook, chapter: u8) -> Option<Self> {
        let result = BibleChapter { book, chapter };
        match result.is_valid() {
            true => Some(result),
            false => None,
        }
    }

    /// Check that the chapter is in the right range
    pub fn is_valid(&self) -> bool {
        let range = 1..=self.book.number_of_chapters();
        range.contains(&(self.chapter as u32))
    }

    /// Attempt to parse a Bible book and chapter from a string
    ///
    /// # Example
    ///
    /// ```rust
    /// use bible_data::{BibleBook, BibleChapter};
    /// assert_eq!(
    ///    BibleChapter::parse("Ge 1"),
    ///    Some(BibleChapter {
    ///        book: BibleBook::Genesis,
    ///        chapter: 1
    ///    })
    /// );
    /// ```
    pub fn parse(text: &str) -> Option<Self> {
        text.parse().ok()
    }
}

impl PartialOrd for BibleChapter {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BibleChapter {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.book.cmp(&other.book) {
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Equal => self.chapter.cmp(&other.chapter),
        }
    }
}

impl FromStr for BibleChapter {
    type Err = ParseChapterError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Start by attempting to parse the book from the abbrev at the start of the text
        // as this is very quick
        match parse_book_abbrev(s) {
            None => Err(NoSuchBookError::new(
                match s.split_once(" ") {
                    None => s,
                    Some(split) => split.0,
                }
                .to_string(),
            )
            .into()),
            Some(index) => {
                let book = BibleBook::from_index(index)
                    .expect("Result of parse_book_abbrev should be in range");
                // Result of parse_book_abbrev ends with end of string or space character
                // We can find rest of strin (if any) by looking for the first space character
                match s.find(" ") {
                    None => {
                        // There is no chapter specified
                        // This is invalid, unless the book only has one chapter
                        // In which case, chapter one is implicit
                        match book.number_of_chapters() {
                            1 => Ok(BibleChapter { book, chapter: 1 }),
                            _ => Err(NoChapterSpecified::new(s.to_string()).into()),
                        }
                    }
                    Some(pos) => {
                        let remain = &s[pos + 1..];
                        // This should be the chapter number
                        match u8::from_str(remain) {
                            Err(_) => Err(NotANumber::new(remain.to_string()).into()),
                            Ok(chapter) if chapter == 0 => Err(ChapterOutOfRange::new(
                                "0. Chapter numbers start at 1".to_string(),
                            )
                            .into()),
                            Ok(chapter) if chapter as u32 > book.number_of_chapters() => {
                                Err(ChapterOutOfRange::new(format!(
                                    "{} has {} chapters. {} is too high.",
                                    book.name(),
                                    book.number_of_chapters(),
                                    chapter
                                ))
                                .into())
                            }
                            Ok(chapter) =>
                            // We have a valid reference!
                            {
                                Ok(BibleChapter { book, chapter })
                            }
                        }
                    }
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
            BibleChapter::parse("Ge 1"),
            Some(BibleChapter {
                book: BibleBook::Genesis,
                chapter: 1
            })
        );
        assert_eq!(BibleChapter::parse("Ge 1:4"), None); // Reference to a verse is not a chapter
        assert_eq!(BibleChapter::parse("Ge +random text"), None);
        assert_eq!(BibleChapter::parse("Just random text"), None);
        assert_eq!(BibleChapter::parse("Ge 51"), None); // No such chapter
        assert_eq!(BibleChapter::parse("Ge"), None); // Missing chapter not allowed
        assert_eq!(
            BibleChapter::parse("3Jn"),
            Some(BibleChapter {
                book: BibleBook::ThirdJohn,
                chapter: 1
            })
        ); // Missing chapter allowed if there is only one chapter
        assert_eq!(
            BibleChapter::parse("3Jn 1"),
            Some(BibleChapter {
                book: BibleBook::ThirdJohn,
                chapter: 1
            })
        ); // But we should also be able to specify it explicitly
    }

    #[test]
    fn test_ord() {
        let gen1 = BibleChapter::new(BibleBook::Genesis, 1);
        let gen2 = BibleChapter::new(BibleBook::Genesis, 2);
        let gen50 = BibleChapter::new(BibleBook::Genesis, 50);
        let ex5 = BibleChapter::new(BibleBook::Exodus, 5);
        let lev1 = BibleChapter::new(BibleBook::Leviticus, 1);

        assert!(gen1 < gen2);
        assert!(gen50 > gen2);
        assert!(ex5 > gen50);
        assert!(gen50 < ex5);
        assert!(gen50 < lev1);
        assert!(gen1 == BibleChapter::new(BibleBook::Genesis, 1));
    }

    #[test]
    fn test_is_valid() {
        assert!(
            BibleChapter {
                book: BibleBook::Genesis,
                chapter: 50
            }
            .is_valid()
        );
        assert!(
            !BibleChapter {
                book: BibleBook::Genesis,
                chapter: 0
            }
            .is_valid()
        );
        assert!(
            !BibleChapter {
                book: BibleBook::Genesis,
                chapter: 51
            }
            .is_valid()
        );
    }

    #[test]
    fn test_new() {
        assert!(BibleChapter::new(BibleBook::Genesis, 50).is_some());
        assert!(BibleChapter::new(BibleBook::Genesis, 0).is_none());
        assert!(BibleChapter::new(BibleBook::Genesis, 51).is_none());
    }

    #[test]
    fn test_display() {
        let display = format!("{}", BibleChapter::new(BibleBook::Eccesiastes, 2).unwrap());
        assert_eq!(display, "Ecc 2")
    }
}
