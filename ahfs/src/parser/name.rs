use std::fmt;

/// Classifies the [`Region`][region] identified by a [`Token`][token].
///
/// [region]: ../source/struct.Region.html
/// [token]: struct.Token.html
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Name {
    // Delimiters.
    AngleLeft,
    AngleRight,
    BraceLeft,
    BraceRight,
    Colon,
    Comma,
    ParenLeft,
    ParenRight,
    Slash,
    SquareLeft,
    SquareRight,
    Semicolon,

    // Literals.
    Boolean,
    Integer,
    Float,
    String,

    // Keywords.
    Consumes,
    Implement,
    Import,
    Interface,
    Method,
    Produces,
    Property,
    Record,
    Service,
    System,
    Using,

    // Other.
    Identifier,
    Comment,
    Error,
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            Name::AngleLeft => "<",
            Name::AngleRight => ">",
            Name::BraceLeft => "{",
            Name::BraceRight => "}",
            Name::Colon => ":",
            Name::Comma => ",",
            Name::ParenLeft => "(",
            Name::ParenRight => ")",
            Name::Slash => "/",
            Name::SquareLeft => "[",
            Name::SquareRight => "]",
            Name::Semicolon => ";",

            Name::Boolean => "Boolean",
            Name::Integer => "Integer",
            Name::Float => "Float",
            Name::String => "String",

            Name::Consumes => "consumes",
            Name::Implement => "implement",
            Name::Import => "import",
            Name::Interface => "interface",
            Name::Method => "method",
            Name::Produces => "produces",
            Name::Property => "property",
            Name::Record => "record",
            Name::Service => "service",
            Name::System => "system",
            Name::Using => "using",

            Name::Identifier => "Identifier",
            Name::Comment => "Comment",
            Name::Error => "Error",
        })
    }
}