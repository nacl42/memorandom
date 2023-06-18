use crate::{cursor::Cursor, Memo};

// Design decision for this first implementation: linewise parsing with result item for the read line.
// In other words, the input stream is not separated into different tokens.

// TODO: proper error output

// TODO: how could we include attributes? |:qty 1
//  maybe use :attribute and +more text and if you do not like +, then use <<EOF notation
// TODO: how to join values separated on multiple lines?
//   YAML: | => keep newlines as newline,  > => replace newlines by spaces, 
// .instructions>
//  do whatever is necessary
//  go on
//  whatever
// .next point
// But | is not really a simple character! On the other hand, it is similar to YAML
// .instructions<<EOF
// .foo,>  (everything is joined into a single string, then the values are split by ,) -- could be default!
// .foo,|  (each line is considered a separate value, with a comma as separator for values in a line)

#[derive(Eq, PartialEq, Debug)]
pub enum LineData<'a> {
    Header {
        schema: &'a str,
        id: Option<&'a str>,
    },
    Data {
        key: &'a str,
        value: Option<&'a str>,
        sep: Option<&'a str>,
    },
    Value {
        value: Option<&'a str>,
    },
    Comment {
        comment: Option<&'a str>,
    },
    Other {
        text: Option<&'a str>,
    },
    Empty,
}

#[derive(Debug, Eq, PartialEq)]
pub enum ParseError {
    // parse
    ExpectedMemo,
    ExpectedData,
    // parse_line
    ExpectedHeaderSchema,
    ExpectedDataField,
    NotImplemented,
    InvalidKeyChar,
}

pub fn parse_line<'a, C>(cursor: C) -> Result<LineData<'a>, ParseError>
where
    C: Into<Cursor<'a>>,
{
    let mut cursor = cursor.into();

    match cursor.pop() {
        Some('@') => {
            // required schema name
            cursor.mark();
            cursor.pop_while(|ch| !ch.is_whitespace());
            let schema = &cursor
                .marked_input()
                .ok_or(ParseError::ExpectedHeaderSchema)?;

            // skip whitespace
            cursor.pop_while(char::is_whitespace);

            // optional id
            let id = cursor.rest_of_line().map(str::trim);

            Ok(LineData::Header { schema, id })
        }
        Some('.') => {
            // required key name
            cursor.mark();
            cursor.pop_while(|ch| ch.is_alphanumeric() || ch == '_' || ch == ':' || ch == '-');
            let key = &cursor.marked_input().ok_or(ParseError::ExpectedDataField)?;

            // optional separator char
            let sep = match cursor.pop() {
                Some(x) if x.is_whitespace() => None, // go on
                Some(',') => Some(","),
                Some(';') => Some(";"),
                Some(x) => return Err(ParseError::InvalidKeyChar),
                None => None, // go on
            };

            // skip whitespace
            cursor.pop_while(char::is_whitespace);

            // optional value
            let value = cursor.rest_of_line(); // value is NOT trimmed
            Ok(LineData::Data { key, value, sep })
        }
        Some(' ') => {
            let value = cursor.rest_of_line();
            Ok(LineData::Value { value })
        }
        Some('#') => {
            let comment = cursor.rest_of_line();
            Ok(LineData::Comment { comment })
        }
        Some(_) => {
            cursor.rewind();
            let text = cursor.rest_of_line();
            Ok(LineData::Other { text })
        }
        None => Ok(LineData::Empty),
    }
}

// this function can later on be changed to an iterator
pub fn parse<'a>(input: &'a str) -> Result<Vec<Memo>, ParseError> {
    let mut memos = vec![];

    let mut current_memo: Option<Memo> = None;
    let mut current_key: Option<&'a str> = None;
    let mut current_sep: Option<&'a str> = None;
    let mut current_values: Vec<&'a str> = vec![];

    for line in input.lines() {
        match parse_line(line) {
            Ok(LineData::Comment { .. }) => { /* skip comment */ }
            Ok(LineData::Header { schema, id }) => {
                /* start new memo */
                if let Some(mut memo) =
                    current_memo.replace(Memo::new(schema, id.unwrap_or_default()))
                {
                    if let Some(key) = current_key.take() {
                        let value = current_values.split_off(0).join("").to_string();
                        memo.insert(key, value);
                    };
                    memos.push(memo);
                }
                current_sep = None;
            }
            Ok(LineData::Data { key, value, sep }) => {
                /* start new data field */
                if current_memo.is_none() {
                    return Err(ParseError::ExpectedMemo);
                };
                if let Some(key) = current_key.replace(key) {
                    let value = current_values.split_off(0).join("").to_string();

                    match current_sep {
                        None => {
                            /* no separator => just a single value */
                            current_memo.as_mut().map(|m| m.insert(key, value));
                        }
                        Some(sep) => {
                            /* separator given => split value into multiple data fields */
                            current_memo.as_mut().map(|m| for v in value.split(sep) { m.insert(key, v.trim())});
                        }
                    }                   
                };
                if let Some(value) = value {
                    current_values.push(value);
                }
                current_sep = sep; 
            }
            Ok(LineData::Value { value }) => {
                /* add value */
                current_values.push(value.unwrap_or_default());
            }
            Ok(LineData::Other { .. }) => {
                // TODO: what should we do with this?
            }
            Ok(LineData::Empty) => {
                // ignore
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    // add current memo
    if let Some(mut memo) = current_memo.take() {
        if let Some(key) = current_key.take() {
            let value = current_values.split_off(0).join("").to_string();
            match current_sep {
                None => {
                    /* no separator => just a single value */
                    memo.insert(key, value);
                }
                Some(sep) => {
                    /* separator given => split value into multiple data fields */
                    for v in value.split(sep) {
                        memo.insert(key, v.trim());
                    }
                }
            }
        }
        memos.push(memo);
    }

    Ok(memos)
}

#[cfg(test)]
mod test_parser {
    use super::*;

    #[test]
    fn test_parse_header_line() {
        let test_cases = [
            ("@", Err(ParseError::ExpectedHeaderSchema)),
            (
                "@book",
                Ok(LineData::Header {
                    schema: "book",
                    id: None,
                }),
            ),
            (
                "@book    ",
                Ok(LineData::Header {
                    schema: "book",
                    id: None,
                }),
            ),
            (
                "@book The Lord of the Rings",
                Ok(LineData::Header {
                    schema: "book",
                    id: Some("The Lord of the Rings"),
                }),
            ),
            (
                "@book The Lord of the Rings    ", // trim trailing spaces for header id
                Ok(LineData::Header {
                    schema: "book",
                    id: Some("The Lord of the Rings"),
                }),
            ),
        ];

        for (input, expected) in test_cases.iter() {
            assert_eq!(parse_line(*input), *expected);
        }
    }

    #[test]
    fn test_parse_data_line() {
        let test_cases = [
            (".", Err(ParseError::ExpectedDataField)),
            (
                ".author",
                Ok(LineData::Data {
                    key: "author",
                    value: None,
                    sep: None,
                }),
            ),
            (
                ".author   ",
                Ok(LineData::Data {
                    key: "author",
                    value: None,
                    sep: None,
                }),
            ),
            (
                ".author J.R.R. Tolkien",
                Ok(LineData::Data {
                    key: "author",
                    value: Some("J.R.R. Tolkien"),
                    sep: None,
                }),
            ),
            (
                ".author J.R.R. Tolkien   ", // keep trailing space for values
                Ok(LineData::Data {
                    key: "author",
                    value: Some("J.R.R. Tolkien   "),
                    sep: None,
                }),
            ),
        ];

        for (input, expected) in test_cases.iter() {
            assert_eq!(parse_line(*input), *expected);
        }
    }

    #[test]
    fn test_parse_value_line() {
        let test_cases = [
            (" ", Ok(LineData::Value { value: None })),
            ("   ", Ok(LineData::Value { value: Some("  ") })),
            (
                " When Mr. Bilbo Baggins of Bag End...",
                Ok(LineData::Value {
                    value: Some("When Mr. Bilbo Baggins of Bag End..."),
                }),
            ),
        ];

        for (input, expected) in test_cases.iter() {
            assert_eq!(parse_line(*input), *expected);
        }
    }

    #[test]
    fn test_parse_comment_line() {
        let test_cases = [
            ("#", Ok(LineData::Comment { comment: None })),
            (
                "# foo",
                Ok(LineData::Comment {
                    comment: Some(" foo"),
                }),
            ),
        ];

        for (input, expected) in test_cases.iter() {
            assert_eq!(parse_line(*input), *expected);
        }
    }

    #[test]
    fn test_parse_other_line() {
        let test_cases = [
            ("-", Ok(LineData::Other { text: Some("-") })),
            (
                "foo bar",
                Ok(LineData::Other {
                    text: Some("foo bar"),
                }),
            ),
        ];

        for (input, expected) in test_cases.iter() {
            assert_eq!(parse_line(*input), *expected);
        }
    }
}
