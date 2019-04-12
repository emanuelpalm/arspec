use super::{Name, Token};
use ::source::{Region, Text};
use std::char;

/// A utility for creating [`Token`s](struct.Token.html) from source texts.
///
/// # Operation
///
/// Extracts _tokens_ from a _source_ text. When created, it contains a
/// _candidate token_ with length 0 at the beginning of its text. The candidate
/// token can be expanded to include more characters, and later collected or
/// discarded when it includes some set of significant characters. If collected,
/// the candidate is returned. If either collected or discarded, a new
/// zero-length candidate is created at the end position of the old one.
#[derive(Debug)]
pub struct Scanner<'a> {
    text: &'a Text,
    bytes: &'a [u8],
    start: usize,
    end: usize,
}

impl<'a> Scanner<'a> {
    /// Creates new `Scanner` from given source code `text`.
    #[inline]
    pub fn new(text: &'a Text) -> Self {
        Scanner {
            text,
            bytes: text.body().as_bytes(),
            start: 0,
            end: 0,
        }
    }

    /// Returns character right after candidate, and then expands candidate.
    pub fn next(&mut self) -> Option<char> {
        let x = self.next_byte()?;
        if x < 128 {
            return Some(unsafe { char::from_u32_unchecked(x as u32) });
        }
        let init = (x & 0x1F) as u32;
        let y = (self.next_byte_or_0() & 0b0011_1111) as u32;
        let mut ch = (init << 6) | y;
        if x >= 0xE0 {
            let z = (self.next_byte_or_0() & 0b0011_1111) as u32;
            let y_z = (y << 6) | z;
            ch = init << 12 | y_z;
            if x >= 0xF0 {
                let w = (self.next_byte_or_0() & 0b0011_1111) as u32;
                ch = (init & 7) << 18 | (y_z << 6) | w;
            }
        }
        return Some(unsafe { char::from_u32_unchecked(ch) });
    }

    #[inline]
    fn next_byte(&mut self) -> Option<u8> {
        let x = self.bytes.get(self.end);
        self.end += 1;
        x.map(|x| *x)
    }

    #[inline]
    fn next_byte_or_0(&mut self) -> u8 {
        self.next_byte().unwrap_or(0)
    }

    /// Undoes last call to `next()`.
    pub fn undo(&mut self) {
        loop {
            if self.end == 0 {
                break;
            }
            self.end -= 1;
            let byte = unsafe { *self.bytes.get_unchecked(self.end) };
            if (byte & 0b1100_0000) != 0b1000_0000 {
                break;
            }
        }
    }

    /// Collects current candidate token.
    #[inline]
    pub fn collect(&mut self, name: Name) -> Token<'a> {
        let token = Token::new(name, unsafe {
            Region::new(self.text, self.start..self.end)
        });
        self.discard();
        token
    }

    /// Discards current candidate token.
    #[inline]
    pub fn discard(&mut self) {
        self.start = self.end;
    }

    /// Returns current candidate string, without consuming or discarding it.
    #[inline]
    pub fn review(&self) -> &str {
        unsafe {
            self.text.body().get_unchecked(self.start..self.end)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collect() {
        let text = Text::new("", "aabbccc");
        let mut reader = Scanner::new(&text);

        // Skip As.
        assert_eq!(Some('a'), reader.next());
        assert_eq!(Some('a'), reader.next());
        reader.discard();

        // Take Bs.
        assert_eq!(Some('b'), reader.next());
        assert_eq!(Some('b'), reader.next());
        let token = reader.collect(Name::Identifier);
        assert_eq!("bb", text.get(token).unwrap());

        // Take Cs.
        assert_eq!(Some('c'), reader.next());
        assert_eq!(Some('c'), reader.next());
        reader.undo();
        let candidate = reader.review();
        assert_eq!("c", candidate);
    }
}
