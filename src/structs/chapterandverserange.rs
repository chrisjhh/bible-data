use crate::structs::errors::{ImplicitRange, InvalidFormat};

use super::chapterandverse::ChapterAndVerse;
use super::chapterandverseorverse::ChapterAndVerseOrVerse;
use super::errors::ParseChapterVeseRangeError;
use std::ops::RangeInclusive;
use std::str::FromStr;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ChapterAndVerseRange(pub RangeInclusive<ChapterAndVerse>);

/// Chapter and Verse ranges may be full or implicit
/// Full ranges specify the chapter at least at the start of the range
/// Implict ranges ommit the chapter and are only valid for books with
/// a single chapter where chapter 1 may be assumed.
/// A single verse is also a valid range (of 1)
/// Examples of full ranges `10:1-5` `8:22-9:1` `11:2`
/// Examples of implicit ranges `1-5` `3`
#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
pub enum FullOrImplicitRange {
    Full(ChapterAndVerseRange),
    Implicit(ChapterAndVerseRange),
}

#[allow(dead_code)]
impl ChapterAndVerseRange {
    pub fn parse(text: &str) -> Option<FullOrImplicitRange> {
        match ChapterAndVerseRange::from_str(text) {
            Ok(cvr) => Some(FullOrImplicitRange::Full(cvr)),
            Err(ParseChapterVeseRangeError::ImplicitRange(ImplicitRange { data })) => {
                Some(FullOrImplicitRange::Implicit(data))
            }
            Err(_) => None,
        }
    }
}

impl FromStr for ChapterAndVerseRange {
    type Err = ParseChapterVeseRangeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.find("-") {
            None => {
                // Single verse range
                match ChapterAndVerseOrVerse::from_str(s)
                    .map_err(|e| e.inner_into::<ParseChapterVeseRangeError>())?
                {
                    ChapterAndVerseOrVerse::Both(cv) => {
                        Ok(ChapterAndVerseRange(cv.clone()..=cv.clone()))
                    }
                    ChapterAndVerseOrVerse::JustVerse(v) => {
                        Err(ImplicitRange::new(ChapterAndVerseRange(
                            ChapterAndVerse::new(1, v)..=ChapterAndVerse::new(1, v),
                        ))
                        .into())
                    }
                }
            }
            Some(pos) => {
                let start = &s[..pos];
                let end = &s[pos + 1..];
                let cvv_start = ChapterAndVerseOrVerse::from_str(start)
                    .map_err(|e| e.inner_into::<ParseChapterVeseRangeError>())?;
                let cvv_end = ChapterAndVerseOrVerse::from_str(end)
                    .map_err(|e| e.inner_into::<ParseChapterVeseRangeError>())?;
                let mut implicit = false;
                let cv_start = match cvv_start {
                    ChapterAndVerseOrVerse::Both(cv) => cv,
                    ChapterAndVerseOrVerse::JustVerse(v) => {
                        implicit = true;
                        ChapterAndVerse {
                            chapter: 1,
                            verse: v,
                        }
                    }
                };
                let cv_end = match cvv_end {
                    ChapterAndVerseOrVerse::Both(cv) => {
                        match implicit {
                            false => cv,
                            true => {
                                return Err(InvalidFormat::new(
                                    "Chapter specified at end only".to_string(),
                                )
                                .into());
                            } // Can't specify chapter at end only!
                        }
                    }
                    ChapterAndVerseOrVerse::JustVerse(v) => ChapterAndVerse {
                        chapter: cv_start.chapter,
                        verse: v,
                    },
                };
                match implicit {
                    false => Ok(ChapterAndVerseRange(cv_start..=cv_end)),
                    true => Err(ImplicitRange::new(ChapterAndVerseRange(cv_start..=cv_end)).into()),
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
        // Full references
        assert_eq!(
            ChapterAndVerseRange::parse("20:1-5").unwrap(),
            FullOrImplicitRange::Full(ChapterAndVerseRange(
                ChapterAndVerse {
                    chapter: 20,
                    verse: 1
                }..=ChapterAndVerse {
                    chapter: 20,
                    verse: 5
                }
            ))
        );
        assert_eq!(
            ChapterAndVerseRange::parse("8:22-9:6").unwrap(),
            FullOrImplicitRange::Full(ChapterAndVerseRange(
                ChapterAndVerse {
                    chapter: 8,
                    verse: 22
                }..=ChapterAndVerse {
                    chapter: 9,
                    verse: 6
                }
            ))
        );
        assert_eq!(
            ChapterAndVerseRange::parse("10:17").unwrap(),
            FullOrImplicitRange::Full(ChapterAndVerseRange(
                ChapterAndVerse {
                    chapter: 10,
                    verse: 17
                }..=ChapterAndVerse {
                    chapter: 10,
                    verse: 17
                }
            ))
        );
        // Implicit references
        assert_eq!(
            ChapterAndVerseRange::parse("1-5").unwrap(),
            FullOrImplicitRange::Implicit(ChapterAndVerseRange(
                ChapterAndVerse {
                    chapter: 1,
                    verse: 1
                }..=ChapterAndVerse {
                    chapter: 1,
                    verse: 5
                }
            ))
        );
        assert_eq!(
            ChapterAndVerseRange::parse("4").unwrap(),
            FullOrImplicitRange::Implicit(ChapterAndVerseRange(
                ChapterAndVerse {
                    chapter: 1,
                    verse: 4
                }..=ChapterAndVerse {
                    chapter: 1,
                    verse: 4
                }
            ))
        );
        // Invalid ranges
        assert_eq!(ChapterAndVerseRange::parse("1-2:2"), None);
        assert_eq!(ChapterAndVerseRange::parse("1:1-"), None);
        assert_eq!(ChapterAndVerseRange::parse("1-"), None);
        assert_eq!(ChapterAndVerseRange::parse(":1-2"), None);
        assert_eq!(ChapterAndVerseRange::parse(":1-2:3"), None);
    }
}
