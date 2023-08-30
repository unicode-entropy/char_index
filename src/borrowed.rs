//! Module containing [`IndexedChars`] and its trait implementations

use crate::IndexedCharsInner;
use core::{
    borrow::Borrow,
    cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd},
    fmt,
    hash::{Hash, Hasher},
    ops::Deref,
};

/// A string whose char indices have been cached for ~O(1) char lookup.  
///
/// This structure allocates 1 additional bytes per unicode scalar value,
/// which in the case of ascii will only use 2 total bytes for a
/// single char (as opposed to the 4 bytes required in `Vec<char>`).
///
/// As the number of non ascii characters increases, the data density will worsen, until the potential worst case of 5 bytes per character.
///
/// The internal representation of this type allows for up to 255 bytes of non ascii unicode chars before an internal rollover occurs (thus tending the complexity towards O(log n)), this is the tradeoff made to reduce memory usage. See the section [`How it Works`](index.html#how-it-works) for details on why char indexing worst case is O(log n), and why in practical cases it appears to be O(1).
///
/// This type mimics a `&'a str` with its trait impls, including `Debug`, `Display`, `PartialEq` with `&str` `PartialOrd` with `&str`, `Hash`, and `AsRef`/`Borrow`.
pub struct IndexedChars<'a> {
    /// Backing string buffer
    buf: &'a str,
    /// Char offsets index
    inner: IndexedCharsInner,
}

impl<'a> IndexedChars<'a> {
    /// Constructs a new [`IndexedChars`] instance from a [`&str`]. This is O(n), but the cost should only be paid once ideally.
    ///
    ///
    /// # Examples
    /// ```rust
    /// # use char_index::IndexedChars;
    /// let index = IndexedChars::new("foo");
    /// # assert_eq!(index.get_char(0), Some('f'));
    /// ```
    #[must_use]
    pub fn new(s: &'a str) -> Self {
        let inner = IndexedCharsInner::new(s);

        Self { buf: s, inner }
    }

    /// Indexes into the backing string to retrieve the nth codepoint.
    ///
    /// This operation has an average case of O(1), and a worst case of O(log n).
    ///
    /// # Examples
    /// ```rust
    /// # use char_index::IndexedChars;
    /// assert_eq!(IndexedChars::new("foobar").get_char(3), Some('b'));
    /// ```
    #[must_use]
    pub fn get_char(&self, index: usize) -> Option<char> {
        self.inner.get_char(self.buf, index)
    }

    /// Returns a reference to the backing `&str`
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.buf
    }
}

// The following lines are all trait implementations made to mirror what str does, and be compatible with str

impl Deref for IndexedChars<'_> {
    type Target = str;

    fn deref(&self) -> &str {
        self.buf
    }
}

impl AsRef<str> for IndexedChars<'_> {
    fn as_ref(&self) -> &str {
        self
    }
}

impl Borrow<str> for IndexedChars<'_> {
    fn borrow(&self) -> &str {
        self
    }
}

impl fmt::Debug for IndexedChars<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <&str as fmt::Debug>::fmt(&self.buf, f)
    }
}

impl fmt::Display for IndexedChars<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <&str as fmt::Display>::fmt(&self.buf, f)
    }
}

impl Eq for IndexedChars<'_> {}

impl PartialEq for IndexedChars<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.buf.eq(other.buf)
    }
}

impl PartialEq<str> for IndexedChars<'_> {
    fn eq(&self, other: &str) -> bool {
        self.buf.eq(other)
    }
}

impl PartialEq<IndexedChars<'_>> for str {
    fn eq(&self, other: &IndexedChars<'_>) -> bool {
        self.eq(other.buf)
    }
}

impl Ord for IndexedChars<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.buf.cmp(other.buf)
    }
}

impl PartialOrd for IndexedChars<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd<str> for IndexedChars<'_> {
    fn partial_cmp(&self, other: &str) -> Option<Ordering> {
        Some((*self.buf).cmp(other))
    }
}

impl PartialOrd<IndexedChars<'_>> for str {
    fn partial_cmp(&self, other: &IndexedChars<'_>) -> Option<Ordering> {
        Some(self.cmp(other.buf))
    }
}

impl Hash for IndexedChars<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.buf.hash(state);
    }
}
