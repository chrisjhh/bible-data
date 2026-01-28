use super::chapterandverse::ChapterAndVerse;
use super::chapterandverseorverse::ChapterAndVerseOrVerse;
use std::ops::RangeInclusive;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
pub struct ChapterAndVerseRange(RangeInclusive<ChapterAndVerse>);

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
        match text.find("-") {
            None => {
                // Single verse range
                match ChapterAndVerseOrVerse::parse(text)? {
                    ChapterAndVerseOrVerse::Both(cv) => {
                        Some(FullOrImplicitRange::Full(ChapterAndVerseRange(
                            ChapterAndVerse {
                                chapter: cv.chapter,
                                verse: cv.verse,
                            }..=ChapterAndVerse {
                                chapter: cv.chapter,
                                verse: cv.verse,
                            },
                        )))
                    }
                    ChapterAndVerseOrVerse::JustVerse(v) => {
                        Some(FullOrImplicitRange::Implicit(ChapterAndVerseRange(
                            ChapterAndVerse {
                                chapter: 1,
                                verse: v,
                            }..=ChapterAndVerse {
                                chapter: 1,
                                verse: v,
                            },
                        )))
                    }
                }
            }
            Some(pos) => {
                let start = &text[..pos];
                let end = &text[pos + 1..];
                let cvv_start = ChapterAndVerseOrVerse::parse(start)?;
                let cvv_end = ChapterAndVerseOrVerse::parse(end)?;
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
                            true => return None, // Can't specify chapter at end only!
                        }
                    }
                    ChapterAndVerseOrVerse::JustVerse(v) => ChapterAndVerse {
                        chapter: cv_start.chapter,
                        verse: v,
                    },
                };
                match implicit {
                    false => Some(FullOrImplicitRange::Full(ChapterAndVerseRange(
                        cv_start..=cv_end,
                    ))),
                    true => Some(FullOrImplicitRange::Implicit(ChapterAndVerseRange(
                        cv_start..=cv_end,
                    ))),
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
