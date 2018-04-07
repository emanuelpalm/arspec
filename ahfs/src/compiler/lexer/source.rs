use std::str;
use super::Lexeme;

/// A utility for creating [`Lexeme`s](struct.Lexeme.html) from source strings.
///
/// # Operation
///
/// Extracts _lexemes_ from a _source_ string. When created, it contains a
/// _candidate lexeme_ with length 0 at the beginning of its source. The
/// candidate lexeme can be expanded to include more bytes, and later collected
/// or discarded when it includes some set of significant characters. If
/// collected, the candidate is returned. If either collected or discarded,
/// a new zero-length candidate is created at the end position of the old one.
#[derive(Debug)]
pub struct Source<'a> {
    source: &'a [u8],
    start: usize,
    end: usize,
}

impl<'a> Source<'a> {
    #[inline]
    pub fn new(source: &'a str) -> Self {
        Source { source: source.as_bytes(), start: 0, end: 0 }
    }

    #[inline]
    fn source(&self) -> &'a str {
        unsafe { str::from_utf8_unchecked(self.source) }
    }

    /// Returns byte right after candidate, and then expands the candidate.
    #[inline]
    pub fn next(&mut self) -> Option<u8> {
        self.source.get(self.end).map(|byte| {
            self.end += 1;
            *byte
        })
    }

    /// Returns byte right after candidate.
    #[inline]
    pub fn peek(&self) -> Option<u8> {
        self.source.get(self.end).map(|byte| *byte)
    }

    /// Expands candidate, making it include one more byte.
    pub fn skip(&mut self) {
        self.end += 1;
    }

    /// Collects current candidate lexeme.
    ///
    /// # Panics
    ///
    /// If the current `start` and `end` offsets do not align with the UTF-8
    /// code boundaries of the source string, the method panics.
    #[inline]
    pub fn collect<K>(&mut self, kind: K) -> Lexeme<'a, K> {
        let lexeme = Lexeme::new(kind, &self.source()[self.start..self.end]);
        self.discard();
        lexeme
    }

    /// Discards current candidate lexeme.
    #[inline]
    pub fn discard(&mut self) {
        self.start = self.end;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collect() {
        let source_str = "aabbcc";
        let mut source = Source::new(source_str);

        // Skip As.
        assert_eq!(Some(b'a'), source.next());
        assert_eq!(Some(b'a'), source.next());
        source.discard();

        // Take Bs.
        assert_eq!(Some(b'b'), source.next());
        assert_eq!(Some(b'b'), source.next());

        let lexeme = source.collect(());
        assert_eq!("bb", lexeme.as_str());
    }
}
