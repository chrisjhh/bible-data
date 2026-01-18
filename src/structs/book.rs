use crate::{BOOK_ABBREVS, BOOK_CHAPTERS, BOOK_NAMES, parse_book_abbrev};
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
#[allow(dead_code)]
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

#[allow(dead_code)]
impl BibleBook {
    /// Return the book number for this book as it occurs in the Bible.
    /// 1 = Genesis. 66 = Revelation.
    ///
    /// # Example
    /// ```rust
    /// use bible_data::BibleBook;
    /// assert_eq!(BibleBook::Genesis.book_number(), 1);
    /// ```
    pub fn book_number(&self) -> u32 {
        *self as u32
    }

    /// Return the zero-based index for accessing arrays
    /// 0 = Genesis. 65 = Revelation
    ///
    /// # Example
    /// ```rust
    /// use bible_data::BibleBook;
    /// assert_eq!(BibleBook::Genesis.index(), 0);
    /// ```
    pub fn index(&self) -> usize {
        self.book_number() as usize - 1
    }

    /// Return the name of this book
    /// eg. "Genesis" or "Revelation"
    ///
    /// # Example
    /// ```rust
    /// use bible_data::BibleBook;
    /// assert_eq!(BibleBook::Genesis.name(), "Genesis");
    /// ```
    pub fn name(&self) -> &str {
        BOOK_NAMES[self.index()]
    }

    /// Return the abbreviation for this book
    /// eg. "Ge" or "Rev"
    ///
    /// # Example
    /// ```rust
    /// use bible_data::BibleBook;
    /// assert_eq!(BibleBook::Genesis.abbrev(), "Ge");
    /// ```
    pub fn abbrev(&self) -> &str {
        BOOK_ABBREVS[self.index()]
    }

    /// Construct a BibleBook from its book number.
    /// 1 = Genesis. 66 = Revelation.
    ///
    /// Returns Ok([BibleBook]) or Err([OutOfRangeError])
    ///
    /// # Example
    /// ```rust
    /// use bible_data::BibleBook;
    /// assert_eq!(BibleBook::from_book_number(1).unwrap(), BibleBook::Genesis);
    /// assert_eq!(BibleBook::from_book_number(66).unwrap(), BibleBook::Revelation);
    /// assert!(BibleBook::from_book_number(0).is_err());
    /// assert!(BibleBook::from_book_number(67).is_err());
    /// ```
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

    /// Construct a BibleBook from its zero-based index.
    /// 0 = Genesis. 65 = Revelation.
    ///
    /// Returns Ok([BibleBook]) or Err([OutOfRangeError])
    ///
    /// # Example
    /// ```rust
    /// use bible_data::BibleBook;
    /// assert_eq!(BibleBook::from_index(1).unwrap(), BibleBook::Exodus);
    /// assert!(BibleBook::from_index(66).is_err());
    /// assert_eq!(BibleBook::from_index(0).unwrap(), BibleBook::Genesis);
    /// assert_eq!(BibleBook::from_index(65).unwrap(), BibleBook::Revelation);
    /// ```
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

    /// Construct a BibleBook instance by parsing its abbreviation.
    ///
    /// Returns an Option that will be None if the abbreviation does not match
    /// that of any known Bible book.
    ///
    /// # Example
    /// ```rust
    /// use bible_data::BibleBook;
    /// assert_eq!(BibleBook::parse_abbrev("Ge").unwrap(), BibleBook::Genesis);
    /// assert!(BibleBook::parse_abbrev("random text").is_none());
    /// ```
    pub fn parse_abbrev(abbrev: &str) -> Option<Self> {
        match parse_book_abbrev(abbrev) {
            None => None,
            Some(index) => Self::from_index(index as u32).ok(),
        }
    }

    /// Construct a BibleBook instance by parsing its name.
    ///
    /// Returns an Option that will be None if the name does not match
    /// that of any known Bible book.
    ///
    /// # Example
    /// ```rust
    /// use bible_data::BibleBook;
    /// assert_eq!(BibleBook::parse_name("Genesis").unwrap(), BibleBook::Genesis);
    /// assert!(BibleBook::parse_abbrev("random text").is_none());
    /// ```
    pub fn parse_name(name: &str) -> Option<Self> {
        match BOOK_NAMES.iter().enumerate().find(|item| *item.1 == name) {
            None => None,
            Some(item) => Self::from_index(item.0 as u32).ok(),
        }
    }

    /// Parse a string into a BibleBook instance
    /// Attempts to match either a valid abbreviation
    /// or the complete book name
    ///
    /// # Example
    /// ```rust
    /// use bible_data::BibleBook;
    /// assert_eq!(BibleBook::parse("Ge").unwrap(), BibleBook::Genesis);
    /// assert_eq!(BibleBook::parse("Genesis").unwrap(), BibleBook::Genesis);
    /// assert!(BibleBook::parse("random text").is_none());
    /// ```
    pub fn parse(value: &str) -> Option<Self> {
        match Self::parse_abbrev(value) {
            Some(value) => Some(value),
            None => Self::parse_name(value),
        }
    }

    /// Return an iterator over all the books of the Bible as BibleBook instances
    ///
    /// # Example
    /// ```rust
    /// use bible_data::BibleBook;
    /// let mut it = BibleBook::iter();
    /// assert_eq!(it.next(), Some(BibleBook::Genesis));
    /// assert_eq!(it.next(), Some(BibleBook::Exodus));
    /// let mut it = it.skip(37);
    /// assert_eq!(it.next(), Some(BibleBook::Matthew));
    /// let mut it = it.skip(25);
    /// assert_eq!(it.next(), Some(BibleBook::Revelation));
    /// assert!(it.next().is_none());
    /// ```
    pub fn iter() -> impl Iterator<Item = BibleBook> {
        (1..=66).map(|number| Self::from_book_number(number).unwrap())
    }

    /// Return if this book is part of the New Testament
    ///
    /// # Example
    ///
    /// ```rust
    /// use bible_data::BibleBook;
    /// assert_eq!(BibleBook::Genesis.is_new_testament(), false);
    /// assert_eq!(BibleBook::Revelation.is_new_testament(), true);
    /// assert_eq!(BibleBook::iter().filter(|b| b.is_new_testament()).count(), 27);
    /// ```
    pub fn is_new_testament(&self) -> bool {
        match self.book_number() {
            40..=66 => true,
            _ => false,
        }
    }

    /// Return if this book is part of the Old Testament
    ///
    /// # Example
    ///
    /// ```rust
    /// use bible_data::BibleBook;
    /// assert_eq!(BibleBook::Genesis.is_old_testament(), true);
    /// assert_eq!(BibleBook::Revelation.is_old_testament(), false);
    /// assert_eq!(BibleBook::iter().filter(|b| b.is_old_testament()).count(), 39);
    /// assert_eq!(BibleBook::iter().filter(|b| b.is_old_testament()).last().unwrap(), BibleBook::Malachi);
    /// ```
    pub fn is_old_testament(&self) -> bool {
        match self.book_number() {
            1..=39 => true,
            _ => false,
        }
    }

    /// Return the number of chapters in this book
    ///
    /// ```rust
    /// use bible_data::BibleBook;
    /// assert_eq!(BibleBook::Daniel.number_of_chapters(), 12);
    /// ```
    pub fn number_of_chapters(&self) -> u32 {
        BOOK_CHAPTERS[self.index()] as u32
    }
}

// TryFrom / TryInto
impl TryFrom<u8> for BibleBook {
    type Error = OutOfRangeError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(Self::from_book_number(value as u32)?)
    }
}

impl TryFrom<&str> for BibleBook {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match Self::parse(value) {
            Some(book) => Ok(book),
            None => Err(()),
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

    #[test]
    fn test_parse_name() {
        assert_eq!(
            BibleBook::parse_name("Genesis").unwrap(),
            BibleBook::Genesis
        );
        assert_eq!(
            BibleBook::parse_name("Malachi").unwrap(),
            BibleBook::Malachi
        );
        assert_eq!(
            BibleBook::parse_name("Matthew").unwrap(),
            BibleBook::Matthew
        );
        assert_eq!(
            BibleBook::parse_name("Matthew").unwrap(),
            BibleBook::Matthew
        );
        assert_eq!(
            BibleBook::parse_name("Revelation").unwrap(),
            BibleBook::Revelation
        );
        assert!(BibleBook::parse_name("1 Maccabees").is_none());
    }

    #[test]
    fn test_iter() {
        let mut it = BibleBook::iter();
        assert_eq!(it.next(), Some(BibleBook::Genesis));
        assert_eq!(it.next(), Some(BibleBook::Exodus));
        let mut it = it.skip(37);
        assert_eq!(it.next(), Some(BibleBook::Matthew));
        let mut it = it.skip(25);
        assert_eq!(it.next(), Some(BibleBook::Revelation));
        assert!(it.next().is_none());
        assert_eq!(BibleBook::iter().count(), 66);
        for (i, book) in BibleBook::iter().enumerate() {
            assert_eq!(book.index(), i);
            assert!((1..=66).contains(&book.book_number()))
        }
    }

    #[test]
    fn test_is_new_testament() {
        assert_eq!(BibleBook::Genesis.is_new_testament(), false);
        assert_eq!(BibleBook::Revelation.is_new_testament(), true);
        assert_eq!(
            BibleBook::iter().filter(|b| b.is_new_testament()).count(),
            27
        );
        for book in BibleBook::iter() {
            assert_eq!(book.is_new_testament(), book.book_number() >= 40);
        }
    }

    #[test]
    fn test_is_old_testament() {
        assert_eq!(BibleBook::Genesis.is_old_testament(), true);
        assert_eq!(BibleBook::Revelation.is_old_testament(), false);
        assert_eq!(
            BibleBook::iter().filter(|b| b.is_old_testament()).count(),
            39
        );
        assert_eq!(
            BibleBook::iter()
                .filter(|b| b.is_old_testament())
                .last()
                .unwrap(),
            BibleBook::Malachi
        );
        for book in BibleBook::iter() {
            assert_eq!(book.is_old_testament(), book.book_number() <= 39);
        }
    }

    #[test]
    fn test_number_of_chapters() {
        assert_eq!(BibleBook::Genesis.number_of_chapters(), 50);
        assert_eq!(BibleBook::Daniel.number_of_chapters(), 12);
        assert_eq!(BibleBook::Revelation.number_of_chapters(), 22);
        assert_eq!(BibleBook::Ephesians.number_of_chapters(), 6);
        let single_chapter_books: Vec<_> = BibleBook::iter()
            .filter(|b| b.number_of_chapters() == 1)
            .map(|b| b.name().to_owned())
            .collect();
        assert_eq!(
            single_chapter_books,
            vec!["Obadiah", "Philemon", "2 John", "3 John", "Jude"]
        );
    }

    #[test]
    fn test_try_into() {
        fn is_single_chapter_book(value: impl TryInto<BibleBook>) -> Option<bool> {
            match value.try_into() {
                Err(_) => None,
                Ok(book) => Some(book.number_of_chapters() == 1),
            }
        }

        assert_eq!(is_single_chapter_book("Ge").unwrap(), false);
        assert_eq!(is_single_chapter_book("Genesis").unwrap(), false);
        assert_eq!(is_single_chapter_book(1).unwrap(), false);
        assert_eq!(is_single_chapter_book(BibleBook::Genesis).unwrap(), false);

        assert_eq!(is_single_chapter_book("3Jn").unwrap(), true);
        assert_eq!(is_single_chapter_book("3 John").unwrap(), true);
        assert_eq!(is_single_chapter_book(65).unwrap(), true);
        assert_eq!(is_single_chapter_book(BibleBook::ThirdJohn).unwrap(), true);

        assert!(is_single_chapter_book("1Macc").is_none());
        assert!(is_single_chapter_book("1 Maccabees").is_none());
        assert!(is_single_chapter_book(67).is_none());
    }

    #[test]
    fn test_ord() {
        // lt
        assert!(BibleBook::Genesis < BibleBook::Exodus);
        assert!(BibleBook::Matthew < BibleBook::Revelation);
        assert!(BibleBook::from_index(2).unwrap() < BibleBook::from_index(7).unwrap());
        assert_eq!(BibleBook::Exodus < BibleBook::Genesis, false);
        // gt
        assert!(BibleBook::Exodus > BibleBook::Genesis);
        assert!(BibleBook::Revelation > BibleBook::Matthew);
        assert!(BibleBook::from_index(7).unwrap() > BibleBook::from_index(2).unwrap());
        assert_eq!(BibleBook::Genesis > BibleBook::Exodus, false);
        // le
        assert!(BibleBook::Genesis <= BibleBook::Exodus);
        assert!(BibleBook::Genesis <= BibleBook::Genesis);
        assert!(BibleBook::Matthew <= BibleBook::Revelation);
        assert!(BibleBook::Matthew <= BibleBook::Matthew);
        assert!(BibleBook::from_index(2).unwrap() <= BibleBook::from_index(7).unwrap());
        assert!(BibleBook::from_index(2).unwrap() <= BibleBook::from_index(2).unwrap());
        assert_eq!(BibleBook::Exodus <= BibleBook::Genesis, false);
        // ge
        assert!(BibleBook::Exodus >= BibleBook::Genesis);
        assert!(BibleBook::Exodus >= BibleBook::Exodus);
        assert!(BibleBook::Revelation >= BibleBook::Matthew);
        assert!(BibleBook::Revelation >= BibleBook::Revelation);
        assert!(BibleBook::from_index(7).unwrap() >= BibleBook::from_index(2).unwrap());
        assert!(BibleBook::from_index(7).unwrap() >= BibleBook::from_index(7).unwrap());
        assert_eq!(BibleBook::Genesis >= BibleBook::Exodus, false);
    }
}
