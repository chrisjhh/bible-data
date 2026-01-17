use crate::{BOOK_ABBREVS, BOOK_NAMES, parse_book_abbrev};
use std::{error::Error, fmt::Display};

/// Error returned when an index or booknumber was used that was out of range
#[derive(Debug)]
pub struct OutOfRangeError {
    message: String,
}

impl Display for OutOfRangeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OutOfRangeError: {}", &self.message)
    }
}
impl Error for OutOfRangeError {}
impl OutOfRangeError {
    pub fn new(message: String) -> Self {
        OutOfRangeError { message }
    }
}

/// Enum representing a book of the bible
/// Uses Rust type safty to ensure the wrapped u8 value is in the correct range
/// BibleBooks can be constructed from an index in the range `0..66` with the method
/// [from_index] or from a book number in the range `1..=66` with [from_book_number]
/// or parsed from an abbreviation with [parse_abbrev] or from the full name with
/// [parse_name]. The method [parse] will try both.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum BibleBook {
    Genesis = 1,
    Exodus,
    Leviticus,
    Numbers,
    Duteronomy,
    Joshua,
    Judges,
    Ruth,
    FirstSamuel,
    SecondSamuel,
    FirstKings,
    SecondKings,
    FirstChronicles,
    SecondChronicles,
    Ezra,
    Nehemiah,
    Esther,
    Job,
    Psalms,
    Proverbs,
    Eccesiastes,
    SongofSongs,
    Isaiah,
    Jeremiah,
    Lamentations,
    Ezekiel,
    Daniel,
    Hosea,
    Joel,
    Amos,
    Obadiah,
    Jonah,
    Micah,
    Nahum,
    Habakkuk,
    Zephaniah,
    Haggai,
    Zechariah,
    Malachi,
    Matthew,
    Mark,
    Luke,
    John,
    Acts,
    Romans,
    FirstCorinthians,
    SecondCorinthians,
    Galatians,
    Ephesians,
    Philippians,
    Colossians,
    FirstThessalonians,
    SecondThessalonians,
    FirstTimothy,
    SecondTimothy,
    Titus,
    Philemon,
    Hebrews,
    James,
    FirstPeter,
    SecondPeter,
    FirstJohn,
    SecondJohn,
    ThirdJohn,
    Jude,
    Revelation,
}

impl BibleBook {
    pub fn book_number(&self) -> u32 {
        *self as u32
    }

    pub fn index(&self) -> u32 {
        self.book_number() - 1
    }

    pub fn name(&self) -> &str {
        BOOK_NAMES[self.index() as usize]
    }

    pub fn abbrev(&self) -> &str {
        BOOK_ABBREVS[self.index() as usize]
    }

    pub fn from_book_number(number: u32) -> Result<Self, OutOfRangeError> {
        match number {
            0 => Err(OutOfRangeError::new(String::from(
                "Zero used. Book numbers start from 1. Did you mean to use from_index()?",
            ))),
            // Calculated use of unsafe code
            // Reasons why it is not really unsafe:
            // 1) We have defined the enum BibleBook with repr(u8) so we know it is a u8 under the hood
            // 2) We are checking it is within the correct limits in this match statement
            // Reasons to use it:
            // An exhaustive match statement for all 66 books would be tedious and *more* error prone
            1..=66 => unsafe { Ok(std::mem::transmute::<u8, BibleBook>(number as u8)) },
            _ => Err(OutOfRangeError::new(String::from(format!(
                "{}. book_number should be in range 1..=66",
                number
            )))),
        }
    }

    pub fn from_index(index: u32) -> Result<Self, OutOfRangeError> {
        match index {
            66 => Err(OutOfRangeError::new(String::from(
                "66 used. Highest value of index is 65. Did you mean to use from_book_number()?",
            ))),
            67.. => Err(OutOfRangeError::new(String::from(format!(
                "{}. index should be in range 0..66",
                index
            )))),
            _ => Self::from_book_number(index + 1),
        }
    }

    pub fn parse_abbrev(abbrev: &str) -> Option<Self> {
        match parse_book_abbrev(abbrev) {
            None => None,
            Some(index) => Self::from_index(index as u32).ok(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_book_number() {
        assert_eq!(BibleBook::Genesis.book_number(), 1);
        assert_eq!(BibleBook::Malachi.book_number(), 39);
        assert_eq!(BibleBook::Matthew.book_number(), 40);
        assert_eq!(BibleBook::Revelation.book_number(), 66);
    }

    #[test]
    fn test_index() {
        assert_eq!(BibleBook::Genesis.index(), 0);
        assert_eq!(BibleBook::Malachi.index(), 38);
        assert_eq!(BibleBook::Matthew.index(), 39);
        assert_eq!(BibleBook::Revelation.index(), 65);
    }

    #[test]
    fn test_name() {
        assert_eq!(BibleBook::Genesis.name(), "Genesis");
        assert_eq!(BibleBook::Malachi.name(), "Malachi");
        assert_eq!(BibleBook::Matthew.name(), "Matthew");
        assert_eq!(BibleBook::Revelation.name(), "Revelation");
    }

    #[test]
    fn test_abbrev() {
        assert_eq!(BibleBook::Genesis.abbrev(), "Ge");
        assert_eq!(BibleBook::Malachi.abbrev(), "Mal");
        assert_eq!(BibleBook::Matthew.abbrev(), "Mt");
        assert_eq!(BibleBook::Revelation.abbrev(), "Rev");
    }

    #[test]
    fn test_from_book_number() {
        assert_eq!(BibleBook::from_book_number(1).unwrap(), BibleBook::Genesis);
        assert_eq!(BibleBook::from_book_number(39).unwrap(), BibleBook::Malachi);
        assert_eq!(BibleBook::from_book_number(40).unwrap(), BibleBook::Matthew);
        assert_eq!(
            BibleBook::from_book_number(66).unwrap(),
            BibleBook::Revelation
        );
        assert!(BibleBook::from_book_number(0).is_err());
        assert!(BibleBook::from_book_number(67).is_err());
    }

    #[test]
    fn test_from_index() {
        assert_eq!(BibleBook::from_index(0).unwrap(), BibleBook::Genesis);
        assert_eq!(BibleBook::from_index(38).unwrap(), BibleBook::Malachi);
        assert_eq!(BibleBook::from_index(39).unwrap(), BibleBook::Matthew);
        assert_eq!(BibleBook::from_index(65).unwrap(), BibleBook::Revelation);
        assert!(BibleBook::from_index(66).is_err());
    }

    #[test]
    fn test_parse_abbrev() {
        assert_eq!(BibleBook::parse_abbrev("Ge").unwrap(), BibleBook::Genesis);
        assert_eq!(BibleBook::parse_abbrev("Mal").unwrap(), BibleBook::Malachi);
        assert_eq!(BibleBook::parse_abbrev("Mt").unwrap(), BibleBook::Matthew);
        assert_eq!(
            BibleBook::parse_abbrev("Rev").unwrap(),
            BibleBook::Revelation
        );
        assert!(BibleBook::parse_abbrev("1Macc").is_none());
    }
}
