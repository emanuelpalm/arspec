use std::fmt;
use super::{LineIter, Lines, Range, Text};

/// Represents a significant region within a borrowed source code text.
#[derive(Clone, Debug)]
pub struct Region<'a> {
    text: &'a Text,
    range: Range,
}

impl<'a> Region<'a> {
    /// It is the responsibility of the caller to ensure given
    /// [`range`](type.Range.html) is within valid UTF-8 bounds of given
    /// [`text`](struct.Text.html).
    #[doc(hidden)]
    #[inline]
    pub unsafe fn new(text: &'a Text, range: Range) -> Self {
        Region { text, range }
    }

    /// Gets string representing only significant range within this `Region`.
    #[inline]
    pub fn as_str(&self) -> &'a str {
        unsafe {
            self.text.body().get_unchecked(self.range.clone())
        }
    }

    /// Byte range of this `Region` within its `text`.
    #[inline]
    pub fn range(&self) -> &Range {
        &self.range
    }

    /// [`Text`](struct.Text.html) in which `Region` is located.
    #[inline]
    pub fn text(&self) -> &'a Text {
        self.text
    }

    /// Creates new `Region` representing only end of this `Region`.
    #[inline]
    pub fn end(&self) -> Region<'a> {
        Region { text: self.text, range: self.range.end..self.range.end }
    }
}

impl<'a> AsRef<str> for Region<'a> {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<'a> fmt::Display for Region<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Lines::fmt(self, f, self.text.name())
    }
}

impl<'a> Lines for Region<'a> {
    fn lines(&self) -> LineIter {
        let body = self.text.body();

        let start = body[..self.range.start]
            .rfind('\n')
            .map(|start| start + 1)
            .unwrap_or(0);

        let end = body[self.range.end..]
            .find('\n')
            .map(|mut end| {
                end += self.range.end;
                if end > 0 && body.as_bytes()[end - 1] == b'\r' {
                    end -= 1;
                }
                end
            })
            .unwrap_or(self.range.end);

        let text = &body[start..end];
        let line_number = body[..start]
            .bytes()
            .filter(|b| *b == b'\n')
            .count() + 1;
        let range = (self.range.start - start)..(self.range.end - start);

        unsafe { LineIter::new(text, line_number, range) }
    }
}

impl<'a, 'b> PartialEq<&'a str> for Region<'b> {
    #[inline]
    fn eq(&self, other: &&'a str) -> bool {
        &self.as_str() == other
    }
}

impl<'a, 'b> PartialEq<Region<'a>> for &'b str {
    #[inline]
    fn eq(&self, other: &Region<'a>) -> bool {
        other == self
    }
}