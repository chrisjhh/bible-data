use crate::structs::errors::NotANumber;

use super::chapterandverse::ChapterAndVerse;
use super::errors::ParseChapterVeseError;
use std::str::FromStr;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
pub enum ChapterAndVerseOrVerse {
    Both(ChapterAndVerse),
    JustVerse(u8),
}

#[allow(dead_code)]
impl ChapterAndVerseOrVerse {
    pub fn parse(text: &str) -> Option<Self> {
        text.parse().ok()
    }
}

impl FromStr for ChapterAndVerseOrVerse {
    type Err = ParseChapterVeseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.find(":") {
            None => u8::from_str(s)
                .map(ChapterAndVerseOrVerse::JustVerse)
                .map_err(|_| NotANumber::new(s.to_string()).into()),
            Some(_) => ChapterAndVerse::from_str(s).map(ChapterAndVerseOrVerse::Both),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            ChapterAndVerseOrVerse::parse("1"),
            Some(ChapterAndVerseOrVerse::JustVerse(1))
        );
        assert_eq!(
            ChapterAndVerseOrVerse::parse("1:1"),
            Some(ChapterAndVerseOrVerse::Both(ChapterAndVerse {
                chapter: 1,
                verse: 1
            }))
        );
        assert_eq!(
            ChapterAndVerseOrVerse::parse("119:176"),
            Some(ChapterAndVerseOrVerse::Both(ChapterAndVerse {
                chapter: 119,
                verse: 176
            }))
        );
        assert_eq!(
            ChapterAndVerseOrVerse::parse("150"),
            Some(ChapterAndVerseOrVerse::JustVerse(150))
        );
        assert_eq!(ChapterAndVerseOrVerse::parse("-1"), None);
        assert_eq!(ChapterAndVerseOrVerse::parse("1:"), None);
        assert_eq!(ChapterAndVerseOrVerse::parse(":1"), None);
        assert_eq!(ChapterAndVerseOrVerse::parse(":"), None);
        assert_eq!(ChapterAndVerseOrVerse::parse(""), None);
    }
}
