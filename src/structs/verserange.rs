use std::fmt::Display;
use std::ops::RangeInclusive;

use super::book::BibleBook;
use super::chapterandverse::ChapterAndVerse;
use super::chapterandverserange::{ChapterAndVerseRange, FullOrImplicitRange};
use super::verse::BibleVerse;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BibleVerseRange {
    pub book: BibleBook,
    pub range: RangeInclusive<ChapterAndVerse>,
}

#[allow(dead_code)]
impl BibleVerseRange {
    pub fn parse(text: &str) -> Option<Self> {
        match BibleBook::parse(&text) {
            None => None,
            Some(book) => match text.find(" ") {
                None => None,
                Some(pos) => {
                    let remain = &text[pos + 1..];
                    match ChapterAndVerseRange::parse(remain) {
                        None => None,
                        Some(FullOrImplicitRange::Full(cvr)) => match cvr.0.is_empty() {
                            true => None,
                            false => Some(BibleVerseRange { book, range: cvr.0 }),
                        },
                        Some(FullOrImplicitRange::Implicit(cvr)) => match book.number_of_chapters()
                        {
                            1 => match cvr.0.is_empty() {
                                true => None,
                                false => Some(BibleVerseRange { book, range: cvr.0 }),
                            },
                            _ => None,
                        },
                    }
                }
            },
        }
    }

    pub fn contains(&self, verse: &BibleVerse) -> bool {
        self.book == verse.book
            && self.range.contains(&ChapterAndVerse {
                chapter: verse.chapter,
                verse: verse.verse,
            })
    }

    pub fn chapters(&self) -> impl Iterator<Item = u8> {
        self.range.start().chapter..=self.range.end().chapter
    }

    pub fn new(
        book: BibleBook,
        start_chapter: u8,
        start_verse: u8,
        end_chapter: u8,
        end_verse: u8,
    ) -> Self {
        BibleVerseRange {
            book,
            range: ChapterAndVerse {
                chapter: start_chapter,
                verse: start_verse,
            }..=ChapterAndVerse {
                chapter: end_chapter,
                verse: end_verse,
            },
        }
    }
}

impl Display for BibleVerseRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let start = if self.book.number_of_chapters() == 1 {
            format!("{}", self.range.start().verse)
        } else {
            format!("{}", self.range.start())
        };
        if self.range.start() == self.range.end() {
            write!(f, "{} {}", self.book.abbrev(), start)
        } else {
            let end = if self.book.number_of_chapters() == 1
                || self.range.start().chapter == self.range.end().chapter
            {
                format!("{}", self.range.end().verse)
            } else {
                format!("{}", self.range.end())
            };
            write!(f, "{} {}-{}", self.book.abbrev(), start, end)
        }
    }
}

impl TryFrom<&str> for BibleVerseRange {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match BibleVerseRange::parse(value) {
            None => Err(()),
            Some(item) => Ok(item),
        }
    }
}

impl From<BibleVerse> for BibleVerseRange {
    fn from(value: BibleVerse) -> Self {
        BibleVerseRange {
            book: value.book,
            range: ChapterAndVerse {
                chapter: value.chapter,
                verse: value.verse,
            }..=ChapterAndVerse {
                chapter: value.chapter,
                verse: value.verse,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            BibleVerseRange::parse("Ge 1:1-10").unwrap(),
            BibleVerseRange::new(BibleBook::Genesis, 1, 1, 1, 10)
        );
        assert_eq!(
            BibleVerseRange::parse("Ge 1:1-1:10").unwrap(),
            BibleVerseRange::new(BibleBook::Genesis, 1, 1, 1, 10)
        );
        assert_eq!(
            BibleVerseRange::parse("Ex 5:3-6:1").unwrap(),
            BibleVerseRange::new(BibleBook::Exodus, 5, 3, 6, 1)
        );
        assert_eq!(
            BibleVerseRange::parse("Lev 7:5").unwrap(),
            BibleVerseRange::new(BibleBook::Leviticus, 7, 5, 7, 5)
        );
        assert_eq!(
            BibleVerseRange::parse("Ob 2-5").unwrap(),
            BibleVerseRange::new(BibleBook::Obadiah, 1, 2, 1, 5)
        );
        assert_eq!(
            BibleVerseRange::parse("Ob 1:2-5").unwrap(),
            BibleVerseRange::new(BibleBook::Obadiah, 1, 2, 1, 5)
        );
        assert_eq!(
            BibleVerseRange::parse("2Jn 3").unwrap(),
            BibleVerseRange::new(BibleBook::SecondJohn, 1, 3, 1, 3)
        );
        assert_eq!(
            BibleVerseRange::parse("2Jn 1:3").unwrap(),
            BibleVerseRange::new(BibleBook::SecondJohn, 1, 3, 1, 3)
        );
        assert_eq!(BibleVerseRange::parse("Ge 10"), None);
        assert_eq!(BibleVerseRange::parse("Ge 10:"), None);
        assert_eq!(BibleVerseRange::parse("Ge 10:10-9"), None);
        assert_eq!(BibleVerseRange::parse("Ge 10:1-9:2"), None);
        assert_eq!(BibleVerseRange::parse("Ob 5-2"), None);
    }

    #[test]
    fn test_display() {
        assert_eq!(
            format!("{}", BibleVerseRange::new(BibleBook::Genesis, 1, 1, 1, 10)),
            "Ge 1:1-10"
        );
        assert_eq!(
            format!("{}", BibleVerseRange::new(BibleBook::Genesis, 1, 1, 1, 1)),
            "Ge 1:1"
        );
        assert_eq!(
            format!("{}", BibleVerseRange::new(BibleBook::Matthew, 5, 20, 6, 5)),
            "Mt 5:20-6:5"
        );
        assert_eq!(
            format!("{}", BibleVerseRange::new(BibleBook::Jude, 1, 2, 1, 5)),
            "Jude 2-5"
        );
        assert_eq!(
            format!("{}", BibleVerseRange::new(BibleBook::Jude, 1, 2, 1, 2)),
            "Jude 2"
        );
    }
}
