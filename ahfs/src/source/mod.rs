//! Various types and utilities related to managing source code texts.

mod error;
mod line;
mod lines;
mod region;
mod text;

pub use self::error::Error;
pub use self::line::Line;
pub use self::lines::Lines;
pub use self::region::Region;
pub use self::text::Text;

use std::io;
use std::ops;
use std::path::PathBuf;
use std::result;

/// Refers to a range of bytes within some arbitrary `str`.
pub type Range = ops::Range<usize>;

/// The result of a source-related compiler operation.
pub type Result<'a, T> = result::Result<T, Error<'a>>;

/// A collection of source code [`Text`s](struct.Text.html).
pub struct Source {
    texts: Box<[Text]>,
}

impl Source {
    /// Creates new `Source` from given `texts`.
    pub fn new<S>(texts: S) -> Self
        where S: Into<Box<[Text]>>
    {
        Source { texts: texts.into() }
    }

    /// Creates new `Source` from file at given `path`.
    pub fn read_file<P>(path: P) -> io::Result<Source>
        where P: Into<PathBuf>,
    {
        Text::read(path).map(|text| Self::new(vec![text]))
    }

    /// Reads contents of files at given `paths` into new `Source` instance.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ahfs::source::Source;
    /// use std::fs;
    ///
    /// let mut paths = fs::read_dir(".").unwrap()
    ///     .filter_map(|entry| {
    ///         let entry = entry.unwrap();
    ///         if !entry.file_type().unwrap().is_file() {
    ///             return None;
    ///         }
    ///         Some(entry.path())
    ///     });
    /// let source = Source::read_files(paths).unwrap();
    /// println!("{:?}", source.texts());
    /// ```
    pub fn read_files<I>(paths: I) -> io::Result<Source>
        where I: IntoIterator,
              I::Item: Into<PathBuf>,
    {
        let mut texts = Vec::new();
        for path in paths {
            texts.push(Text::read(path)?);
        }
        Ok(Source::new(texts))
    }

    /// `Source` texts.
    #[inline]
    pub fn texts(&self) -> &[Text] {
        &self.texts
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        let texts = vec![
            Text::new("alpha.ahs", concat!(
                "A type System;\n",
                "A consumes B;\r\n",
                "A produces C;\n",
            )),
            Text::new("beta.ahs", concat!(
                "X",
            )),
        ];
        let source = Source::new(texts);
        let get = |index: usize, range: Range| {
            format!("{}", source.texts()
                .get(index).unwrap()
                .get_region(range)
                .unwrap())
        };
        {
            assert_eq!(get(1, 0..1).as_str(), concat!(
                "      : ", str_color!(blue: "beta.ahs"), "\n",
                "      |\n",
                "    1 | ", str_color!(none: "X"), "\n",
                "      | ", str_color!( red: "^"), "\n"));
        }
        {
            assert_eq!(get(0, 0..1).as_str(), concat!(
                "      : ", str_color!(blue: "alpha.ahs"), "\n",
                "      |\n",
                "    1 | ", str_color!(none: "A type System;"), "\n",
                "      | ", str_color!( red: "^"), "\n"));
        }
        {
            assert_eq!(get(0, 17..25).as_str(), concat!(
                "      : ", str_color!(blue: "alpha.ahs"), "\n",
                "      |\n",
                "    2 | ", str_color!(none: "A consumes B;"), "\n",
                "      | ", str_color!( red: "  ^^^^^^^^"), "\n"));
        }
        {
            assert_eq!(get(0, 30..42).as_str(), concat!(
                "      : ", str_color!(blue: "alpha.ahs"), "\n",
                "      |\n",
                "    3 | ", str_color!(none: "A produces C;"), "\n",
                "      | ", str_color!( red: "^^^^^^^^^^^^"), "\n"));
        }
        {
            assert_eq!(get(0, 17..40).as_str(), concat!(
                "      : ", str_color!(blue: "alpha.ahs"), "\n",
                "      |\n",
                "    2 | ", str_color!(none: "A consumes B;"), "\n",
                "      | ", str_color!( red: "  ^^^^^^^^^^^"), "\n",
                "    3 | ", str_color!(none: "A produces C;"), "\n",
                "      | ", str_color!( red: "^^^^^^^^^^"), "\n"));
        }
        {
            assert_eq!(get(0, 7..40).as_str(), concat!(
                "      : ", str_color!(blue: "alpha.ahs"), "\n",
                "      |\n",
                "    1 | ", str_color!(none: "A type System;"), "\n",
                "      | ", str_color!( red: "       ^^^^^^^"), "\n",
                "    2 | ", str_color!(none: "A consumes B;"), "\n",
                "      | ", str_color!( red: "^^^^^^^^^^^^^"), "\n",
                "     ...\n"));
        }
        {
            assert_eq!(get(0, 42..42).as_str(), concat!(
                "      : ", str_color!(blue: "alpha.ahs"), "\n",
                "      |\n",
                "    3 | ", str_color!(none: "A produces C;"), "\n",
                "      | ", str_color!( red: "            ^"), "\n"));
        }
    }
}