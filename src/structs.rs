//! Mod for the structs contained in this crate
//! It is intended that these be entirely optional and not needed for the use of the data
//! provided in this crate.
//!
//! However they are useful for leveraging the type-safety of Rust if you are writing APIs
//! that need to take valid bible books or references. Of course you are also free to use your
//! own. (This is another reason why the data and methods in the root of the crate don't use
//! them.)
//!
//! These do not need to be under an optional package. Just use them if you need them
//! and ignore them if you don't. Let the linker do the work to include what is needed in
//! the final binaries.
pub mod book;
pub mod chapter;
