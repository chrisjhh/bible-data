use super::chapterandverserange::ChapterAndVerseRange;
use std::error::Error;
use std::fmt::Display;

macro_rules! create_error {
    ($error_name:ident) => {
        #[allow(dead_code)]
        #[derive(Debug, Clone)]
        pub struct $error_name {
            message: String,
        }
        impl Display for $error_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:?}: {}", stringify!($error_name), &self.message)
            }
        }
        impl Error for $error_name {}

        #[allow(dead_code)]
        impl $error_name {
            pub fn new(message: String) -> Self {
                $error_name { message }
            }
        }
    };
    ($error_name:ident<$data_type:ident>) => {
        #[allow(dead_code)]
        #[derive(Debug, Clone)]
        pub struct $error_name {
            pub data: $data_type,
        }
        impl Display for $error_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:?}: {:?}", stringify!($error_name), &self.data)
            }
        }
        impl Error for $error_name {}

        #[allow(dead_code)]
        impl $error_name {
            pub fn new(data: $data_type) -> Self {
                $error_name { data }
            }
        }
    };
    ($error_type:ident : $($sub_type:ident),+) => {
        #[allow(dead_code)]
        #[derive(Debug)]
        pub enum $error_type {
            $($sub_type($sub_type)),+
        }
        impl Display for $error_type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $($error_type::$sub_type(e) => e.fmt(f)),+
                }
            }
        }
        impl Error for $error_type {}
        $(
            impl From<$sub_type> for $error_type {
                fn from(value: $sub_type) -> Self {
                    $error_type::$sub_type(value)
                }
            }
        )+
        impl $error_type {
             #[allow(dead_code)]
            pub fn inner_into<T>(&self) -> T
            where T: $(From<$sub_type> +)+
            {
                match self {
                    $($error_type::$sub_type(e) => (*e).clone().into()),+
                }
            }
        }
    }
}

create_error!(OutOfRangeError);
create_error!(NoSuchBookError);
create_error!(NoChapterSpecified);
create_error!(NotANumber);
create_error!(ChapterOutOfRange);
create_error!(InvalidFormat);
create_error!(ImplicitRange<ChapterAndVerseRange>);

create_error!(ParseChapterError : NoSuchBookError, NoChapterSpecified, NotANumber, ChapterOutOfRange);
create_error!(ParseChapterVeseError: NotANumber, InvalidFormat);
create_error!(ParseChapterVeseRangeError: NotANumber, InvalidFormat, ImplicitRange);
