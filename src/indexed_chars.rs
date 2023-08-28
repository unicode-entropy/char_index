//! Houses core implementation of char index.

use alloc::vec::Vec;

/// The core type of `char_index`.
/// This struct implements building a memory efficient index of char
///  locations, and a method to access that index.
#[derive(Debug)]
pub(crate) struct IndexedCharsInner {
    /// The char offsets, stores the amount that a given char index must increment by to be in the correct range
    chars: Vec<u8>,
    /// rollovers, stores the points where the offsets overflowed u8, so it may be binary searched to add `u8::MAX` * index_in_rollovers to the offset
    rollovers: Vec<usize>,
}

impl IndexedCharsInner {
    /// Computes a new char index from a backing string
    pub(crate) fn new(s: &str) -> Self {
        // this is expensive but it lets us avoid big reallocs
        // it also lets us niche on ascii strings
        let charlen = s.chars().count();

        // if the number of chars is equal to the number of bytes we can skip allocating at all
        // this lets us niche on an ascii string
        if charlen == s.len() {
            return Self {
                chars: Vec::new(),
                rollovers: Vec::new(),
            };
        }

        let mut chars = Vec::with_capacity(charlen);
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

        // ensure we did not waste memory
        debug_assert!(chars.capacity() == chars.len());

        Self { chars, rollovers }
    }

    /// Gets a char from a string using the index, the string passed must be the one this index was created with
    pub(crate) fn get_char(&self, buf: &str, index: usize) -> Option<char> {
        // niche on empty chars with nonempty buf (ascii optimization)
        if self.chars.is_empty() & !buf.is_empty() {
            return buf.get(index..).and_then(|c| c.chars().next());
        }

        // if its in self.chars we can assume its in buf
        let mut offset = usize::from(*self.chars.get(index)?);

        offset += usize::from(u8::MAX)
            * self
                .rollovers
                .binary_search(&index)
                // we inc by 1 if variant is Ok as we want to do the rollover of the
                // index where it "would" be regardless if its found, never its actual location
                .map_or_else(|e| e, |t| t + 1);

        buf[index + offset..].chars().next()
    }
}

#[cfg(test)]
extern crate std;

#[test]
fn create() {
    use alloc::format;

    let s_buf = "foo";
    let s = IndexedCharsInner::new(s_buf);

    assert!(s.rollovers.is_empty());

    let special = '💯';

    let foo_alloc = format!("{special}a");

    let foo = IndexedCharsInner::new(&foo_alloc);

    assert_eq!(foo.chars, &[0, (special.len_utf8() - 1) as u8]);
}

#[test]
fn get_idx() {
    use alloc::string::String;
    use rand::{seq::SliceRandom, thread_rng};

    let mut chars: Vec<_> = (0..20_000)
        .map(|i| char::from_u32(i as u32).unwrap())
        .cycle()
        .take(1_000_000)
        .collect();

    chars.shuffle(&mut thread_rng());

    let s = String::from_iter(&chars);

    let index = IndexedCharsInner::new(&s);

    for (char_idx, (_real_idx, c)) in s.char_indices().enumerate() {
        assert_eq!(index.get_char(&s, char_idx).unwrap(), c);
    }
}
