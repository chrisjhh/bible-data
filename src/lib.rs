pub static BOOK_NAMES: [&str; 66] = [
    "Genesis",
    "Exodus",
    "Leviticus",
    "Numbers",
    "Duteronomy",
    "Joshua",
    "Judges",
    "Ruth",
    "1 Samuel",
    "2 Samuel",
    "1 Kings",
    "2 Kings",
    "1 Chronicles",
    "2 Chronicles",
    "Ezra",
    "Nehemiah",
    "Esther",
    "Job",
    "Psalms",
    "Proverbs",
    "Eccesiastes",
    "Song of Songs",
    "Isaiah",
    "Jeremiah",
    "Lamentations",
    "Ezekiel",
    "Daniel",
    "Hosea",
    "Joel",
    "Amos",
    "Obadiah",
    "Jonah",
    "Micah",
    "Nahum",
    "Habakkuk",
    "Zephaniah",
    "Haggai",
    "Zechariah",
    "Malachi",
    "Matthew",
    "Mark",
    "Luke",
    "John",
    "Acts",
    "Romans",
    "1 Corinthians",
    "2 Corinthians",
    "Galatians",
    "Ephesians",
    "Philippians",
    "Colossians",
    "1 Thessalonians",
    "2 Thessalonians",
    "1 Timothy",
    "2 Timothy",
    "Titus",
    "Philemon",
    "Hebrews",
    "James",
    "1 Peter",
    "2 Peter",
    "1 John",
    "2 John",
    "3 John",
    "Jude",
    "Revelation",
];
pub static BOOK_ABBREVS: [&str; 66] = [
    "Ge", "Ex", "Lev", "Nu", "Dt", "Jos", "Jdg", "Ru", "1Sa", "2Sa", "1Ki", "2Ki", "1Ch", "2Ch",
    "Ezr", "Ne", "Est", "Job", "Ps", "Pr", "Ecc", "SS", "Isa", "Jer", "La", "Eze", "Da", "Hos",
    "Joel", "Am", "Ob", "Jnh", "Mic", "Na", "Hab", "Zep", "Hag", "Zec", "Mal", "Mt", "Mk", "Lk",
    "Jn", "Ac", "Ro", "1Co", "2Co", "Gal", "Eph", "Php", "Col", "1Th", "2Th", "1Ti", "2Ti", "Tit",
    "Phm", "Heb", "Jas", "1Pe", "2Pe", "1Jn", "2Jn", "3Jn", "Jude", "Rev",
];

macro_rules! some_at_end {
    ($chars:ident, $val:literal) => {
        match $chars.next() {
            None => Some($val),
            Some(' ') => Some($val),
            Some(_) => None,
        }
    };
    ($chars:ident, $val:literal, $opt:literal) => {
        match $chars.next() {
            None => Some($val),
            Some(' ') => Some($val),
            Some($opt) => some_at_end!($chars, 0),
            _ => None,
        }
    };
}

pub fn parse_book_abbrev(text: &str) -> Option<usize> {
    let mut chars = text.chars();
    match chars.next()? {
        'M' => match chars.next()? {
            't' => some_at_end!(chars, 39),
            'k' => some_at_end!(chars, 40),
            'a' => match chars.next()? {
                'l' => some_at_end!(chars, 38),
                _ => None,
            },
            'i' => match chars.next()? {
                'c' => some_at_end!(chars, 32),
                _ => None,
            },
            _ => None,
        },
        'L' => match chars.next()? {
            'k' => some_at_end!(chars, 41),
            'e' => match chars.next()? {
                'v' => some_at_end!(chars, 2),
                _ => None,
            },
            'a' => some_at_end!(chars, 24, 'm'),
            _ => None,
        },
        'J' => match chars.next()? {
            'n' => match chars.next() {
                None => Some(42),
                Some(' ') => Some(42),
                Some('h') => some_at_end!(chars, 31),
                _ => None,
            },
            'a' => match chars.next()? {
                's' => some_at_end!(chars, 58),
                _ => None,
            },
            'o' => match chars.next()? {
                's' => some_at_end!(chars, 5),
                'b' => some_at_end!(chars, 17),
                'e' => match chars.next()? {
                    'l' => some_at_end!(chars, 28),
                    _ => None,
                },
                _ => None,
            },
            'd' => match chars.next()? {
                'g' => some_at_end!(chars, 6),
                _ => None,
            },
            'e' => match chars.next()? {
                'r' => some_at_end!(chars, 23),
                _ => None,
            },
            'u' => match chars.next()? {
                'd' => match chars.next()? {
                    'e' => some_at_end!(chars, 64),
                    _ => None,
                },
                _ => None,
            },
            _ => None,
        },
        'E' => match chars.next()? {
            'p' => match chars.next()? {
                'h' => some_at_end!(chars, 48),
                _ => None,
            },
            'x' => some_at_end!(chars, 1),
            'z' => match chars.next()? {
                'r' => some_at_end!(chars, 14),
                'e' => some_at_end!(chars, 25),
                _ => None,
            },
            's' => match chars.next()? {
                't' => some_at_end!(chars, 16),
                _ => None,
            },
            'c' => match chars.next()? {
                'c' => some_at_end!(chars, 20),
                _ => None,
            },
            _ => None,
        },
        'G' => match chars.next()? {
            'e' => some_at_end!(chars, 0, 'n'),
            'a' => match chars.next()? {
                'l' => some_at_end!(chars, 47),
                _ => None,
            },
            _ => None,
        },
        'P' => match chars.next()? {
            's' => some_at_end!(chars, 18),
            'r' => some_at_end!(chars, 19),
            'h' => match chars.next()? {
                'p' => some_at_end!(chars, 49),
                'm' => some_at_end!(chars, 56),
                _ => None,
            },
            _ => None,
        },
        'C' => match chars.next()? {
            'o' => match chars.next()? {
                'l' => some_at_end!(chars, 50),
                _ => None,
            },
            _ => None,
        },
        'R' => match chars.next()? {
            'o' => some_at_end!(chars, 44),
            'e' => match chars.next()? {
                'v' => some_at_end!(chars, 65),
                _ => None,
            },
            'u' => some_at_end!(chars, 7),
            _ => None,
        },
        'H' => match chars.next()? {
            'e' => match chars.next()? {
                'b' => some_at_end!(chars, 57),
                _ => None,
            },
            'a' => match chars.next()? {
                'b' => some_at_end!(chars, 34),
                'g' => some_at_end!(chars, 36),
                _ => None,
            },
            'o' => match chars.next()? {
                's' => some_at_end!(chars, 27),
                _ => None,
            },
            _ => None,
        },
        'N' => match chars.next()? {
            'u' => some_at_end!(chars, 3, 'm'),
            'e' => some_at_end!(chars, 15),
            'a' => some_at_end!(chars, 33),
            _ => None,
        },
        'D' => match chars.next()? {
            'a' => some_at_end!(chars, 26, 'n'),
            't' => some_at_end!(chars, 4),
            _ => None,
        },
        '1' => match chars.next()? {
            'S' => match chars.next()? {
                'a' => some_at_end!(chars, 8),
                _ => None,
            },
            'K' => match chars.next()? {
                'i' => some_at_end!(chars, 10),
                _ => None,
            },
            'C' => match chars.next()? {
                'h' => some_at_end!(chars, 12),
                'o' => some_at_end!(chars, 45),
                _ => None,
            },
            'T' => match chars.next()? {
                'h' => some_at_end!(chars, 51),
                'i' => some_at_end!(chars, 53),
                _ => None,
            },
            'P' => match chars.next()? {
                'e' => some_at_end!(chars, 59),
                _ => None,
            },
            'J' => match chars.next()? {
                'n' => some_at_end!(chars, 61),
                _ => None,
            },
            _ => None,
        },
        '2' => match chars.next()? {
            'S' => match chars.next()? {
                'a' => some_at_end!(chars, 9),
                _ => None,
            },
            'K' => match chars.next()? {
                'i' => some_at_end!(chars, 11),
                _ => None,
            },
            'C' => match chars.next()? {
                'h' => some_at_end!(chars, 13),
                'o' => some_at_end!(chars, 46),
                _ => None,
            },
            'T' => match chars.next()? {
                'h' => some_at_end!(chars, 52),
                'i' => some_at_end!(chars, 54),
                _ => None,
            },
            'P' => match chars.next()? {
                'e' => some_at_end!(chars, 60),
                _ => None,
            },
            'J' => match chars.next()? {
                'n' => some_at_end!(chars, 62),
                _ => None,
            },
            _ => None,
        },
        'I' => match chars.next()? {
            's' => match chars.next()? {
                'a' => some_at_end!(chars, 22),
                _ => None,
            },
            _ => None,
        },
        'A' => match chars.next()? {
            'c' => some_at_end!(chars, 43),
            'm' => some_at_end!(chars, 29),
            _ => None,
        },
        'Z' => match chars.next()? {
            'e' => match chars.next()? {
                'c' => some_at_end!(chars, 37),
                'p' => some_at_end!(chars, 35),
                _ => None,
            },
            _ => None,
        },
        'T' => match chars.next()? {
            'i' => match chars.next()? {
                't' => some_at_end!(chars, 55),
                _ => None,
            },
            _ => None,
        },
        '3' => match chars.next()? {
            'J' => match chars.next()? {
                'n' => some_at_end!(chars, 63),
                _ => None,
            },
            _ => None,
        },
        'S' => match chars.next()? {
            'S' => some_at_end!(chars, 21),
            'o' => some_at_end!(chars, 21, 'S'),
            _ => None,
        },
        'O' => match chars.next()? {
            'b' => some_at_end!(chars, 30),
            _ => None,
        },
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_book_abbrev() {
        for i in 0..66 {
            let abbrev = BOOK_ABBREVS[i];
            let book_index = parse_book_abbrev(abbrev).unwrap();
            assert_eq!(book_index, i, "Incorrect index for {}", abbrev);
            let abbrev_with_space = abbrev.to_string() + " ";
            let book_index = parse_book_abbrev(&abbrev_with_space).unwrap();
            assert_eq!(book_index, i, "Incorrect index for [{}]", abbrev);
            let abbrev_with_q = abbrev.to_string() + "q";
            let book_index = parse_book_abbrev(&abbrev_with_q);
            assert!(book_index.is_none());
        }
        let random_text = "Hello World!";
        let book_index = parse_book_abbrev(&random_text);
        assert!(book_index.is_none());
    }
}

mod structs;
pub use structs::book;
pub use structs::book::BibleBook;
