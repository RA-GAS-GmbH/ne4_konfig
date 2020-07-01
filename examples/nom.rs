use nom::{
    bytes::complete::{is_not, tag},
    combinator::opt,
    IResult,
};

#[derive(Debug, PartialEq, Eq)]
struct TreeStoreValues<'a> {
    reg: isize,        // Rwreg Nr. (Fcode: 0x03, 0x06)
    range: &'a str,    // Wertebereich
    value: &'a str,    // Zugeordnete Größe und Einheit
    property: &'a str, // Messwerteigenschaft
}

fn parse_treestore_values(input: &str) -> IResult<&str, TreeStoreValues> {
    let (input, reg) = opt(is_not(";"))(input)?;
    let (input, _) = opt(tag(";"))(input)?;

    let (input, range) = opt(is_not(";"))(input)?;
    let (input, _) = opt(tag(";"))(input)?;

    let (input, value) = opt(is_not(";"))(input)?;
    let (input, _) = opt(tag(";"))(input)?;

    let (input, property) = opt(is_not(";"))(input)?;
    let (input, _) = opt(tag(";"))(input)?;

    // extract reg
    let reg = if let Some(reg) = reg {
        match reg.parse() {
            Ok(reg) => reg,
            Err(_) => -1isize,
        }
    } else {
        -1isize
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
            Ok((
                "",
                TreeStoreValues {
                    reg: -1,
                    range: "",
                    value: "",
                    property: "",
                }
            ))
        );
    }

    #[test]
    fn test_invalid_string1() {
        assert_eq!(
            parse_treestore_values("ABC;;;"),
            Ok((
                "",
                TreeStoreValues {
                    reg: -1,
                    range: "",
                    value: "",
                    property: "",
                }
            ))
        );
    }

    #[test]
    // unparsed string returned
    fn test_invalid_string2() {
        assert_eq!(
            parse_treestore_values("x;A;B;C;unparsed;String"),
            Ok((
                "unparsed;String",
                TreeStoreValues {
                    reg: -1,
                    range: "A",
                    value: "B",
                    property: "C",
                }
            ))
        );
    }

    #[test]
    // unparsed string returned
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
}
