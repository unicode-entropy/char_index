#![no_std]
#![warn(clippy::pedantic)]
#![forbid(unsafe_code)]
//#![warn(missing_docs)] TODO enable
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
//! current index we are working with, and then apply the length of the list at that point multiplied by u8::MAX to find the true offset of the character.
//!
//! In this way, we achieve what behaves as an O(1) char lookup (technically worst case O(log n)) for most strings, while saving memory over `Vec<char>` (sans the case where the string is only made up of 4 byte characters, which acts as the worst case for time complexity too).
//!
//! Additionally, as a niche optimization, if the string contains only ascii (all offsets 0); it will simply not allocate any extra memory, and gain perfect O(1) lookup.
//!

extern crate alloc;
use alloc::string::String;

mod indexed_chars;
use indexed_chars::IndexedCharsInner;

/// A string whose char indices have been cached for ~O(1) char lookup.  
///
/// This structure allocates 1 additional bytes per unicode scalar value,
/// which in the case of ascii will only use 2 total bytes for a
/// single char (as opposed to the 4 bytes required in `Vec<char>`).
///
/// As the number of non ascii characters increases, the data density will worsen, until the potential worst case of 5 bytes per character.
///
/// The internal representation of this type allows for up to 255 bytes of non ascii unicode chars before an internal rollover occurs (thus tending the complexity towards O(log n)), this is the tradeoff made to reduce memory usage. See the section [`How it Works`] for details on why char indexing worst case is O(log n), and why in practical cases it appears to be O(1).
#[derive(Debug)]
pub struct IndexedChars<'a> {
    buf: &'a str,
    inner: IndexedCharsInner,
}

impl<'a> IndexedChars<'a> {
    pub fn new(s: &'a str) -> Self {
        let inner = IndexedCharsInner::new(s);

        Self { buf: s, inner }
    }

    pub fn get_char(&self, index: usize) -> Option<char> {
        self.inner.get_char(self.buf, index)
    }
}

/// A string whose char indices have been cached for ~O(1) char lookup. Owned variant.
///
/// This structure allocates 1 additional bytes per unicode scalar value,
/// which in the case of ascii will only use 2 total bytes for a
/// single char (as opposed to the 4 bytes required in `Vec<char>`).
///
/// As the number of non ascii characters increases, the data density will worsen, until the potential worst case of 5 bytes per character.
///
/// The internal representation of this type allows for up to 255 bytes of non ascii unicode chars before an internal rollover occurs (thus tending the complexity towards O(log n)), this is the tradeoff made to reduce memory usage. See the section [`How it Works`] for details on why char indexing worst case is O(log n), and why in practical cases it appears to be O(1).
pub struct OwnedIndexedChars {
    buf: String,
    inner: IndexedCharsInner,
}

impl OwnedIndexedChars {
    pub fn new(s: String) -> Self {
        let inner = IndexedCharsInner::new(&s);

        Self { buf: s, inner }
    }

    pub fn get_char(&self, index: usize) -> Option<char> {
        self.inner.get_char(&self.buf, index)
    }
}
