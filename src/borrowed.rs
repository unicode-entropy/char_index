use crate::IndexedCharsInner;

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
