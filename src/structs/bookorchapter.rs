use std::fmt::Display;

use super::book::BibleBook;
use super::chapter::BibleChapter;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
pub enum BibleBookOrChapter {
    Book(BibleBook),
    Chapter(BibleChapter),
}

impl Display for BibleBookOrChapter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BibleBookOrChapter::Book(book) => write!(f, "{}", book.abbrev()),
            BibleBookOrChapter::Chapter(chapt) => write!(f, "{}", chapt),
        }
    }
}

#[allow(dead_code)]
impl BibleBookOrChapter {
    pub fn parse(text: &str) -> Option<Self> {
        match text.find(" ") {
            None => BibleBook::parse(text).map(BibleBookOrChapter::Book),
            Some(_) => BibleChapter::parse(text).map(BibleBookOrChapter::Chapter),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use BibleBook::*;
    use BibleBookOrChapter::*;

    #[test]
    fn test_parse() {
        assert_eq!(BibleBookOrChapter::parse("Ge"), Some(Book(Genesis)));
        assert_eq!(
            BibleBookOrChapter::parse("Ge 1"),
            Some(Chapter(BibleChapter {
                book: Genesis,
                chapter: 1
            }))
        );
        assert_eq!(BibleBookOrChapter::parse("Ge 51"), None);
        assert_eq!(BibleBookOrChapter::parse("random text"), None);
    }

    #[test]
    fn test_display() {
        for text in vec!["Ge", "Ge 1", "Ro 12", "Rev", "Rev 20"] {
            let boc = BibleBookOrChapter::parse(text).unwrap();
            let display = format!("{}", boc);
            assert_eq!(display, text);
        }
    }
}
