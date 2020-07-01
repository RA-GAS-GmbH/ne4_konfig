use nom::{
    bytes::complete::{is_not, tag},
    combinator::opt,
    IResult,
};

#[derive(Debug, PartialEq, Eq)]
struct TreeStoreValues {
    reg: isize,       // Rwreg Nr. (Fcode: 0x03, 0x06)
    range: String,    // Wertebereich
    value: String,    // Zugeordnete Größe und Einheit
    property: String, // Messwerteigenschaft
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
    let range = if let Some(range) = range {
        range.to_string()
    } else {
        "".to_string()
    };

    // extract value
    let value = if let Some(value) = value {
        value.to_string()
    } else {
        "".to_string()
    };

    // extract property
    let property = if let Some(property) = property {
        property.to_string()
    } else {
        "".to_string()
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
                range: "0 .. 65535 [0]".to_string(),
                value: "".to_string(),
                property: "Kundencode: zur freien Belegung z.B. Raumcode *".to_string(),
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
                    range: "ABC 123 [](){}".to_string(),
                    value: "".to_string(),
                    property: "Just a random String with Symbols *()_!@#".to_string(),
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
                    range: "0 .. 65535 [0]".to_string(),
                    value: "".to_string(),
                    property: "Kundencode: zur freien Belegung z.B. Raumcode *".to_string(),
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
                    range: "0 … 10000 [11111]".to_string(),
                    value: "0 … 10000 ppm".to_string(),
                    property: "Messwertvorgabe für Testzwecke".to_string(),
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
                    range: "".to_string(),
                    value: "".to_string(),
                    property: "".to_string(),
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
                    range: "".to_string(),
                    value: "".to_string(),
                    property: "".to_string(),
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
                    range: "A".to_string(),
                    value: "B".to_string(),
                    property: "C".to_string(),
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
                    range: "A".to_string(),
                    value: "".to_string(),
                    property: "".to_string(),
                }
            ))
        );
    }
}
