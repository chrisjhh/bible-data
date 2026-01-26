use std::cmp::Ordering;
use std::{fmt::Display, str::FromStr};

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
pub struct ChapterAndVerse {
    chapter: u8,
    verse: u8,
}

impl Display for ChapterAndVerse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.chapter, self.verse)
    }
}

impl PartialOrd for ChapterAndVerse {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ChapterAndVerse {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.chapter.cmp(&other.chapter) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => self.verse.cmp(&other.verse),
        }
    }
}

#[allow(dead_code)]
impl ChapterAndVerse {
    pub fn parse(text: &str) -> Option<Self> {
        match text.find(":") {
            None => None,
            Some(pos) => {
                let before = &text[..pos];
                let after = &text[pos + 1..];
                let chapter = u8::from_str(before).ok()?;
                let verse = u8::from_str(after).ok()?;
                Some(ChapterAndVerse { chapter, verse })
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
            ChapterAndVerse::parse("1:1"),
            Some(ChapterAndVerse {
                chapter: 1,
                verse: 1
            })
        );
        assert_eq!(
            ChapterAndVerse::parse("119:176"),
            Some(ChapterAndVerse {
                chapter: 119,
                verse: 176
            })
        );
        assert_eq!(ChapterAndVerse::parse("-1:1"), None);
        assert_eq!(ChapterAndVerse::parse("1:-1"), None);
        assert_eq!(ChapterAndVerse::parse("1;1"), None);
        assert_eq!(ChapterAndVerse::parse("1:"), None);
        assert_eq!(ChapterAndVerse::parse(":1"), None);
        assert_eq!(ChapterAndVerse::parse(":"), None);
        assert_eq!(ChapterAndVerse::parse(""), None);
    }

    #[test]
    fn test_display() {
        assert_eq!(
            format!(
                "{}",
                ChapterAndVerse {
                    chapter: 1,
                    verse: 1
                }
            ),
            "1:1"
        );
        assert_eq!(
            format!(
                "{}",
                ChapterAndVerse {
                    chapter: 119,
                    verse: 176
                }
            ),
            "119:176"
        );
    }

    #[test]
    fn test_ord() {
        let cv1 = ChapterAndVerse {
            chapter: 1,
            verse: 1,
        };
        let cv2 = ChapterAndVerse {
            chapter: 1,
            verse: 10,
        };
        let cv3 = ChapterAndVerse {
            chapter: 2,
            verse: 1,
        };
        let cv4 = ChapterAndVerse {
            chapter: 1,
            verse: 1,
        };
        assert!(cv1 < cv2);
        assert!(cv3 > cv2);
        assert!(cv3 > cv1);
        assert!(cv4 == cv1);
    }
}
