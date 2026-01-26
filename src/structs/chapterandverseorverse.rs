use super::chapterandverse::ChapterAndVerse;
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
        match text.find(":") {
            None => match u8::from_str(text) {
                Err(_) => None,
                Ok(val) => Some(ChapterAndVerseOrVerse::JustVerse(val)),
            },
            Some(_) => match ChapterAndVerse::parse(text) {
                None => None,
                Some(cv) => Some(ChapterAndVerseOrVerse::Both(cv)),
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
