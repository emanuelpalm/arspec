//! Lexical analysis utilities.
//!
//! This module provide various utilities for analyzing UTF-8 source strings.

mod lexeme;
mod lexer;

pub use self::lexeme::Lexeme;
pub use self::lexer::Lexer;

macro_rules! next_or_break {
    ($lexer:ident) => (match $lexer.next() { Some(ch) => ch, None => break });
}

/// Performs default lexical analysis of given source string.
pub fn analyze<'a>(source: &'a str) -> Vec<Lexeme<'a>> {
    let mut lexer = Lexer::new(source);
    let mut ch: char;
    loop {
        ch = next_or_break!(lexer);

        if is_control(ch) {
            lexer.discard();
            continue;
        }

        if is_delimiter(ch) {
            lexer.collect();
            continue;
        }

        loop {
            ch = next_or_break!(lexer);
            if is_control(ch) || is_delimiter(ch) {
                lexer.undo();
                break;
            }
        }
        lexer.collect();
    }
    return lexer.into_lexemes();

    #[inline]
    fn is_control(ch: char) -> bool {
        match ch {
            '\x00'...'\x09' | '\x0b'...' ' | '\x7f' => true,
            _ => false,
        }
    }

    #[inline]
    fn is_delimiter(ch: char) -> bool {
        match ch {
            '\n' | '(' | ')' | ':' | ';' | '[' | ']' | '{' | '}' => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn analyze() {
        const SOURCE: &'static str = concat!(
            "A is: System;\n",
            "B is: Service;\n",
            "# Emojis 😜🤖💩!");

        assert_eq!(
            vec!["A", "is", ":", "System", ";", "\n",
                 "B", "is", ":", "Service", ";", "\n",
                 "#", "Emojis", "😜🤖💩!"],
            super::analyze(SOURCE)
                .iter()
                .map(|lexeme| lexeme.as_str())
                .collect::<Vec<_>>());
    }
}