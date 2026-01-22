use super::book::BibleBook;
use crate::parse_book_abbrev;
use std::str::FromStr;

/// A struct representing a chapter in the Bible
/// Contains the [BibleBook] and the chapter number
#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
pub struct BibleChapter {
    pub book: BibleBook,
    pub chapter: u8,
}

#[allow(dead_code)]
impl BibleChapter {
    /// Construct a new BibleChapter from the book and chapter
    pub fn new(book: BibleBook, chapter: u8) -> Self {
        BibleChapter { book, chapter }
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
        // Start by attempting to parse the book from the abbrev at the start of the text
        // as this is very quick
        match parse_book_abbrev(text) {
            None => None,
            Some(index) => {
                let book = BibleBook::from_index(index)
                    .expect("Result of parse_book_abbrev should be in range");
                // Result of parse_book_abbrev ends with end of string or space character
                // We can find rest of strin (if any) by looking for the first space character
                match text.find(" ") {
                    None => {
                        // There is no chapter specified
                        // This is invalid, unless the book only has one chapter
                        // In which case, chapter one is implicit
                        match book.number_of_chapters() {
                            1 => Some(BibleChapter { book, chapter: 1 }),
                            _ => None,
                        }
                    }
                    Some(pos) => {
                        let remain = &text[pos + 1..];
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
        assert!(BibleChapter::new(BibleBook::Genesis, 50).is_valid());
        assert!(!BibleChapter::new(BibleBook::Genesis, 0).is_valid());
        assert!(!BibleChapter::new(BibleBook::Genesis, 51).is_valid());
    }
}
