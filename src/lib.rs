#![no_std]
#![forbid(unsafe_code)]
#![warn(clippy::pedantic, clippy::missing_docs_in_private_items, missing_docs)]
#![allow(clippy::module_name_repetitions)]
//! # `char_index`
//! A crate that provides a tradeoff of space efficiency and apparent O(1) charwise indexing.  
//!
//! To get started, create a new [`IndexedChars`] or [`OwnedIndexedChars`] instance.
//!
//! # How it Works
//! [`IndexedChars`] works by allocating a `Vec<u8>` under the hood that stores char
//!  offsets from its own index in the Vec to the location of the char in the backing string.
//! This takes advantage of the fact that utf8's minimum size is at least 1 byte per character.
//!
//! This has a tradeoff where after 255 bytes worth of string data that is not a single byte or,
//! the sum of all the characters that exceed one bytes lengths minus the first byte, has been added
//! to the string, it can no longer store data, as it would overflow u8.
//! This is where the rollovers structure takes effect, it is a `Vec<usize>` whose sole purpose is to
//! store indexes where a rollover has occurred, this can then be trivially binary searched to find the
//! current index we are working with, and then apply the length of the list at that point multiplied by `u8::MAX` to find the true offset of the character.
//!
//! In this way, we achieve what behaves as an O(1) char lookup (technically worst case O(log n)) for most strings, while saving memory over `Vec<char>` (sans the case where the string is only made up of 4 byte characters, which acts as the worst case for time complexity too).
//!
//! Additionally, as a niche optimization, if the string contains only ascii (all offsets 0); it will simply not allocate any extra memory, and gain perfect O(1) lookup.
//!

extern crate alloc;

mod indexed_chars;
use indexed_chars::IndexedCharsInner;

mod borrowed;
mod owned;

pub use borrowed::IndexedChars;
pub use owned::OwnedIndexedChars;
