use nom::{
    bytes::complete::{is_not, tag},
    combinator::opt,
    error::{ErrorKind, ParseError},
    Err::Error,
    IResult,
};
use std::error;
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
pub enum NomError<I> {
    NoRegisterFound,
    RegisterInvalid,
    Nom(I, ErrorKind),
}

impl<I> fmt::Display for NomError<I> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NomError::NoRegisterFound => write!(f, "No Register found"),
            NomError::RegisterInvalid => write!(f, "Register value invalid"),
            NomError::Nom(_, _) => write!(f, "Nom Error"),
        }
    }
}

impl<I> error::Error for NomError<I>
where
    I: fmt::Debug,
{
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            NomError::NoRegisterFound => None,
            NomError::RegisterInvalid => None,
            NomError::Nom(_, _) => None,
        }
    }
}

impl<I> From<ParseIntError> for NomError<I> {
    fn from(_: ParseIntError) -> NomError<I> {
        NomError::RegisterInvalid
    }
}

impl<I> ParseError<I> for NomError<I> {
    fn from_error_kind(input: I, kind: ErrorKind) -> Self {
        NomError::Nom(input, kind)
    }

    fn append(_: I, _: ErrorKind, other: Self) -> Self {
        other
    }
}

#[derive(Debug, PartialEq, Eq)]
struct TreeStoreValues<'a> {
    reg: usize,        // Rwreg Nr. (Fcode: 0x03, 0x06)
    range: &'a str,    // Wertebereich
    value: &'a str,    // Zugeordnete Größe und Einheit
    property: &'a str, // Messwerteigenschaft
}

fn parse_treestore_values(input: &str) -> IResult<&str, TreeStoreValues, NomError<&str>> {
    let (input, reg) = opt(is_not(";"))(input)?;
    let (input, _) = opt(tag(";"))(input)?;

    let (input, range) = opt(is_not(";"))(input)?;
    let (input, _) = opt(tag(";"))(input)?;

    let (input, value) = opt(is_not(";"))(input)?;
    let (input, _) = opt(tag(";"))(input)?;

    let (input, property) = opt(is_not(";"))(input)?;
    let (input, _) = opt(tag(";"))(input)?;

    let reg = match reg {
        Some(reg) => reg,
        None => return Err(Error(NomError::NoRegisterFound)),
    };

    let reg = match reg.parse() {
        Ok(reg) => reg,
        Err(_) => return Err(Error(NomError::RegisterInvalid)),
    };

    // extract range
    let range = if let Some(range) = range { range } else { "" };

    // extract value
    let value = if let Some(value) = value { value } else { "" };

    // extract property
    let property = if let Some(property) = property {
        property
    } else {
        ""
    };

    Ok((
        input,
        TreeStoreValues {
            reg,
            range,
            value,
            property,
        },
    ))
}

fn main() {
    assert_eq!(
        parse_treestore_values("0;0 .. 65535 [0];;Kundencode: zur freien Belegung z.B. Raumcode *"),
        Ok((
            "",
            TreeStoreValues {
                reg: 0,
                range: "0 .. 65535 [0]",
                value: "",
                property: "Kundencode: zur freien Belegung z.B. Raumcode *",
            }
        ))
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_string() {
        assert_eq!(
            parse_treestore_values("0;ABC 123 [](){};;Just a random String with Symbols *()_!@#"),
            Ok((
                "",
                TreeStoreValues {
                    reg: 0,
                    range: "ABC 123 [](){}",
                    value: "",
                    property: "Just a random String with Symbols *()_!@#",
                }
            ))
        );
    }

    #[test]
    fn test_real_string1() {
        assert_eq!(
            parse_treestore_values(
                "0;0 .. 65535 [0];;Kundencode: zur freien Belegung z.B. Raumcode *"
            ),
            Ok((
                "",
                TreeStoreValues {
                    reg: 0,
                    range: "0 .. 65535 [0]",
                    value: "",
                    property: "Kundencode: zur freien Belegung z.B. Raumcode *",
                }
            ))
        );
    }

    #[test]
    fn test_real_string2() {
        assert_eq!(
            parse_treestore_values(
                "2;0 … 10000 [11111];0 … 10000 ppm;Messwertvorgabe für Testzwecke"
            ),
            Ok((
                "",
                TreeStoreValues {
                    reg: 2,
                    range: "0 … 10000 [11111]",
                    value: "0 … 10000 ppm",
                    property: "Messwertvorgabe für Testzwecke",
                }
            ))
        );
    }

    #[test]
    fn test_real_string_all_empty() {
        assert_eq!(
            parse_treestore_values(";;;"),
            Err(Error(NomError::NoRegisterFound))
        );
    }

    #[test]
    fn test_string_long() {
        assert_eq!(
            parse_treestore_values("0;A;B;C;unparsed;String"),
            Ok((
                "unparsed;String",
                TreeStoreValues {
                    reg: 0,
                    range: "A",
                    value: "B",
                    property: "C",
                }
            ))
        );
    }

    #[test]
    fn test_invalid_string1() {
        assert_eq!(
            parse_treestore_values("ABC;;;"),
            Err(Error(NomError::RegisterInvalid))
        );
    }

    #[test]
    fn test_invalid_string_to_short() {
        assert_eq!(
            parse_treestore_values("1;A"),
            Ok((
                "",
                TreeStoreValues {
                    reg: 1,
                    range: "A",
                    value: "",
                    property: "",
                }
            ))
        );
    }

    #[test]
    fn test_invalid_empty_string() {
        assert_eq!(
            parse_treestore_values(""),
            Err(Error(NomError::NoRegisterFound))
        );
    }
}
