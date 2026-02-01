use std::fmt::Display;
use std::ops::RangeInclusive;
use std::str::FromStr;

use super::errors::{InvalidRange, NoChapterSpecified, NoSuchBookError, ParseError};

use super::book::BibleBook;
use super::chapterandverse::ChapterAndVerse;
use super::chapterandverserange::ChapterAndVerseRange;
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
        text.parse().ok()
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
            range: ChapterAndVerse::new(start_chapter, start_verse)
                ..=ChapterAndVerse::new(end_chapter, end_verse),
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
    type Error = ParseError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl From<BibleVerse> for BibleVerseRange {
    fn from(value: BibleVerse) -> Self {
        BibleVerseRange {
            book: value.book,
            range: ChapterAndVerse::new(value.chapter, value.verse)
                ..=ChapterAndVerse::new(value.chapter, value.verse),
        }
    }
}

impl FromStr for BibleVerseRange {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let book = BibleBook::parse_abbrev(s)
            .ok_or_else(|| NoSuchBookError::new("No matching abbreviation".to_string()))?;
        match s.find(" ") {
            None => Err(NoChapterSpecified::new("No chapter/verse specified.".to_string()).into()),
            Some(pos) => {
                let remain = &s[pos + 1..];
                match ChapterAndVerseRange::from_str(remain) {
                    Ok(cvr) => match cvr.0.is_empty() {
                        true => Err(InvalidRange::new("End verse before start".to_string()).into()),
                        false => Ok(BibleVerseRange { book, range: cvr.0 }),
                    },
                    Err(ParseError::ImplicitRange(e)) => {
                        let cvr = e.data();
                        match book.number_of_chapters() {
                            1 => match cvr.0.is_empty() {
                                true => {
                                    Err(InvalidRange::new("End verse before start".to_string())
                                        .into())
                                }
                                false => Ok(BibleVerseRange { book, range: cvr.0 }),
                            },
                            _ => Err(NoChapterSpecified::new(
                                "Chapter can only be ommited for single-chapter books".to_string(),
                            )
                            .into()),
                        }
                    }
                    Err(e) => Err(e),
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

    #[test]
    fn test_contains() {
        let book = BibleBook::Genesis;
        let range = BibleVerseRange::new(book, 4, 10, 5, 2);
        assert!(!range.contains(&BibleVerse::new(book, 4, 9)));
        assert!(range.contains(&BibleVerse::new(book, 4, 10)));
        assert!(range.contains(&BibleVerse::new(book, 4, 11)));
        assert!(range.contains(&BibleVerse::new(book, 5, 1)));
        assert!(range.contains(&BibleVerse::new(book, 5, 2)));
        assert!(!range.contains(&BibleVerse::new(book, 5, 3)));
        assert!(!range.contains(&BibleVerse::new(BibleBook::Exodus, 4, 11)));
    }

    #[test]
    fn test_chapters() {
        let range = BibleVerseRange::new(BibleBook::Ezekiel, 4, 10, 6, 1);
        let mut it = range.chapters();
        assert_eq!(it.next().unwrap(), 4);
        assert_eq!(it.next().unwrap(), 5);
        assert_eq!(it.next().unwrap(), 6);
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_try_from() {
        fn num_chapters(item: impl TryInto<BibleVerseRange>) -> Option<usize> {
            let range = item.try_into().ok()?;
            Some(range.chapters().count())
        }

        assert_eq!(num_chapters("Ge 1:2-3:5").unwrap(), 3);
        assert_eq!(
            num_chapters(BibleVerse::new(BibleBook::Acts, 10, 1)).unwrap(),
            1
        );
    }
}
