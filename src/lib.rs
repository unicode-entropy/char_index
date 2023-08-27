#![no_std]
#![warn(clippy::pedantic)]
#![forbid(unsafe_code)]
//#![warn(missing_docs)] TODO enable
//! # `char_index`
//! A crate that provides a tradeoff of space efficiency and apparent O(1) charwise indexing.  
//!
//! To get started, create a new [`IndexedChars`] instance.
//!
//! # How it Works
//! [`IndexedChars`] works by allocating a `Vec<u8>` under the hood that stores char
//!  offsets from its own index in the Vec to the location of the char in the backing string.
//! This takes advantage of the fact that utf8's minimum size is at least 1 byte per character.
//!
//! This has a tradeoff where after 255 bytes worth of string data that is not a single byte or,
//! the sum of all the characters that exceed one bytes lengths minus the first byte, has been added
//! to the string, it can no longer store data, as it would overflow u8.
//! This is where the rollovers structure takes effect, it is a `Vec<u32>` whose sole purpose is to
//! store indexes where a rollover has occurred, this can then be trivially binary searched to find the
//! current index we are working with, and then apply the length of the list at that point multiplied by u8::MAX to find the true offset of the character.
//!
//! In this way, we achieve what behaves as an O(1) char lookup (technically worst case O(log n)) for most strings, while saving memory over `Vec<char>` (sans the case where the string is only made up of 4 byte characters, which acts as the worst case for time complexity too).
//!

#![allow(dead_code)] // TODO remove

extern crate alloc;
use alloc::vec::Vec;

/// A string whose char indices have been cached for ~O(1) char lookup.  
///
/// This structure allocates 1 additional bytes per unicode scalar value,
/// which in the case of ascii will only use 2 total bytes for a
/// single char (as opposed to the 4 bytes required in `Vec<char>`).
///
/// As the number of non ascii characters increases, the data density will worsen, until the potential worst case of 5 bytes per character.
///
/// The internal representation of this type allows for up to 255 bytes of non ascii unicode chars before an internal rollover occurs (thus tending the complexity towards O(log n)), this is the tradeoff made to reduce memory usage. See the section [`How it Works`] for details on why char indexing worst case is O(log n), and why in practical cases it appears to be O(1).
///
/// No string passed to this type may be more than 4 gigabytes.
#[derive(Debug)]
pub struct IndexedChars<'a> {
    buf: &'a str,
    chars: Vec<u8>,
    rollovers: Vec<usize>,
}

impl<'a> IndexedChars<'a> {
    pub fn new(s: &'a str) -> Self {
        let mut chars = Vec::new();
        let mut rollovers = Vec::new();

        for (char_idx, (real_idx, _)) in s.char_indices().enumerate() {
            let mut offset_idx = real_idx - char_idx;

            let u8_max = usize::from(u8::MAX);

            offset_idx -= rollovers.len() * u8_max;

            if offset_idx > u8_max {
                rollovers.push(char_idx);
                offset_idx -= u8_max;

                debug_assert!(offset_idx <= u8_max);
            }

            // unwrap safe as chars cannot grow by more than 255 bytes at once
            // and we just checked if it was over 255 bytes, conditionally subtracting
            chars.push(offset_idx.try_into().unwrap());
        }

        Self {
            buf: s,
            chars,
            rollovers,
        }
    }

    pub fn get(&self, index: usize) -> Option<char> {
        // if its in self.chars we can assume its in buf
        let mut offset = usize::from(*self.chars.get(index)?);

        offset += usize::from(u8::MAX)
            * self
                .rollovers
                .binary_search(&index)
                // we inc by 1 if variant is Ok as we want to do the rollover of the
                // index where it "would" be regardless if its found, never its actual location
                .map_or_else(|e| e, |t| t + 1);

        self.buf[index + offset..].chars().next()
    }
}

#[cfg(test)]
extern crate std;

#[test]
fn create() {
    use alloc::format;

    let s = IndexedChars::new("foo");

    assert_eq!(s.chars, &[0, 0, 0]);
    assert!(s.rollovers.is_empty());

    let special = '💯';

    let foo_alloc = format!("{special}a");

    let foo = IndexedChars::new(&foo_alloc);

    assert_eq!(foo.chars, &[0, (special.len_utf8() - 1) as u8]);
}

#[test]
fn get_idx() {
    use alloc::string::String;
    use rand::{seq::SliceRandom, thread_rng};

    let mut chars: Vec<_> = (0..10_000)
        .map(|i| char::from_u32(i as u32).unwrap())
        .cycle()
        .take(100_000)
        .collect();

    chars.shuffle(&mut thread_rng());

    let s = String::from_iter(&chars);

    let index = IndexedChars::new(&s);

    for (char_idx, (_real_idx, c)) in s.char_indices().enumerate() {
        assert_eq!(index.get(char_idx).unwrap(), c);
    }
}
