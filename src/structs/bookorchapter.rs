use super::book::BibleBook;
use super::chapter::BibleChapter;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
pub enum BibleBookOrChapter {
    Book(BibleBook),
    Chapter(BibleChapter),
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
}
