use std::collections::HashMap;

use crate::{cursor::Cursor, Memo};

// Design decision for this first implementation: linewise parsing with result item for the read line.
// In other words, the input stream is not separated into different tokens.

// TODO: Parsing context: current_header_or_value_or_attribute (=current_item)

//  This would allow multi-line headings, which are not useful as a reference but might
//  be useful for memos that do not contain other information.
//  @memo This
//   would then be possible

// It would also allow parsing attributes for a memo header, which would make the memo header similar to a regular node.
// @memo my-memo
// +id 652176c6-8efe-4ecd-b822-c5000aa1f0aa

// .elements, H, Li, Na
// .elements H, Li, Na
// +mr:split ,
// .elements H and Li and Na
// +mr:split and

#[derive(Eq, PartialEq, Debug)]
pub enum LineData<'a> {
    Header {
        schema: &'a str,
        id: Option<&'a str>,
    },
    Node {
        key: &'a str,
        value: Option<&'a str>,
        sep: Option<&'a str>,
        join: Option<&'a str>,
    },
    Attribute {
        key: &'a str,
        value: Option<&'a str>,
    },
    Continuation {
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
    ExpectedNode,
    // parse_line
    ExpectedHeaderSchema,
    ExpectedNodeField,
    ExpectedAttributeKey,
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
            let key = &cursor.marked_input().ok_or(ParseError::ExpectedNodeField)?;

            // optional separator char
            let (sep, join) = match cursor.pop() {
                Some(',') => (Some(","), Some(",")),
                Some(';') => (Some(";"), Some(";")),
                Some('>') => (None, Some(" ")), // TODO: shouldn't this be the default? Do we really need it?
                Some('|') => (None, Some("\n")),
                Some('*') => (Some("\n"), Some("\n")),
                Some(x) if x.is_whitespace() => (None, None),
                Some(x) => return Err(ParseError::InvalidKeyChar),
                None => (None, None), // default case
            };

            // skip whitespace
            cursor.pop_while(char::is_whitespace);

            // optional value
            let value = cursor.rest_of_line(); // value is NOT trimmed
            Ok(LineData::Node {
                key,
                value,
                sep,
                join,
            })
        }
        Some(' ') => {
            let value = cursor.rest_of_line();
            Ok(LineData::Continuation { value })
        }
        Some('+') => {
            // required key name
            cursor.mark();
            cursor.pop_while(|ch| ch.is_alphanumeric() || ch == '_' || ch == ':' || ch == '-');
            let key = &cursor
                .marked_input()
                .ok_or(ParseError::ExpectedAttributeKey)?;

            // skip whitespace
            cursor.pop_while(char::is_whitespace);

            // optional value
            let value = cursor.rest_of_line(); // value is NOT trimmed
            Ok(LineData::Attribute { key, value })
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
    let mut current_join: Option<&'a str> = None;
    let mut current_values: Vec<&'a str> = vec![];
    let mut current_attributes: HashMap<&'a str, Option<&'a str>> = Default::default();
    let mut current_attribute: Option<&'a str> = None;

    // TODO: finish current memo, maybe as part of ParsingContext
    // fn ctx.finish(self) -> Option<Memo>;

    for line in input.lines() {
        match parse_line(line) {
            Ok(LineData::Comment { .. }) => { /* skip comment */ }
            Ok(LineData::Header { schema, id }) => {
                /* start new memo */
                if let Some(mut memo) =
                    current_memo.replace(Memo::new(schema, id.unwrap_or_default()))
                {
                    if let Some(key) = current_key.take() {
                        let join_str = current_sep.unwrap_or(" ");
                        let value = current_values.split_off(0).join(join_str).to_string();
                        // TODO: add Attributes to each value
                        memo.insert(key, value);
                    };
                    memos.push(memo);
                }
                current_sep = None;
                current_join = None;
                current_attributes = Default::default();
                current_attribute = None;
            }
            Ok(LineData::Node {
                key,
                value,
                sep,
                join,
            }) => {
                /* start new data field */
                if current_memo.is_none() {
                    return Err(ParseError::ExpectedMemo);
                };
                if let Some(key) = current_key.replace(key) {
                    let join = current_join.unwrap_or(" "); // default merge mode
                    let value = current_values.split_off(0).join(join).to_string();

                    match current_sep {
                        None => {
                            /* no separator => just a single value */
                            current_memo.as_mut().map(|m| m.insert(key, value));
                        }
                        Some(sep) => {
                            /* separator given => split value into multiple data fields */
                            current_memo.as_mut().map(|m| {
                                for v in value.split(sep) {
                                    let v = v.trim();
                                    if !v.is_empty() {
                                        m.insert(key, v);
                                    }
                                }
                            });
                        }
                    }
                    // TODO: add attributes to all values
                };
                if let Some(value) = value {
                    current_values.push(value);
                }
                current_sep = sep;
                current_join = join;
            }
            Ok(LineData::Attribute {key, value }) => {
                /* add attribute */
                current_attributes.insert(key, value);
            },
            Ok(LineData::Continuation { value }) => {
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
            let join = current_join.unwrap_or(" "); // default merge char
            let value = current_values.split_off(0).join(join).to_string();
            match current_sep {
                None => {
                    /* no separator => just a single value */
                    memo.insert(key, value);
                }
                Some(sep) => {
                    /* separator given => split value into multiple data fields */
                    for v in value.split(sep) {
                        let v = v.trim();
                        if !v.is_empty() {
                            memo.insert(key, v);
                        }
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
    use std::fmt::Result;

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
            (".", Err(ParseError::ExpectedNodeField)),
            (
                ".author",
                Ok(LineData::Node {
                    key: "author",
                    value: None,
                    sep: None,
                    join: None,
                }),
            ),
            (
                ".author   ",
                Ok(LineData::Node {
                    key: "author",
                    value: None,
                    sep: None,
                    join: None,
                }),
            ),
            (
                ".author J.R.R. Tolkien",
                Ok(LineData::Node {
                    key: "author",
                    value: Some("J.R.R. Tolkien"),
                    sep: None,
                    join: None,
                }),
            ),
            (
                ".author J.R.R. Tolkien   ", // keep trailing space for values
                Ok(LineData::Node {
                    key: "author",
                    value: Some("J.R.R. Tolkien   "),
                    sep: None,
                    join: None,
                }),
            ),
            (
                ".opening>",
                Ok(LineData::Node {
                    key: "opening",
                    value: None,
                    sep: None,
                    join: Some(" "),
                }),
            ),
            (
                ".poem|",
                Ok(LineData::Node {
                    key: "poem",
                    value: None,
                    sep: None,
                    join: Some("\n"),
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
            (" ", Ok(LineData::Continuation { value: None })),
            ("   ", Ok(LineData::Continuation { value: Some("  ") })),
            (
                " When Mr. Bilbo Baggins of Bag End...",
                Ok(LineData::Continuation {
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

    #[test]
    fn test_parse_memo() {
        let run_tests = |cases| {
            let mut last_memo = None;
            for case in cases {
                let memo = parse(case);
                assert!(memo.is_ok());
                if let Some(last_memo) = last_memo {
                    assert_eq!(last_memo, memo);
                };
                last_memo = Some(memo);
            }
        };

        let split_values = vec![
            "@case\n.color red\n.color blue\n.color green",
            "@case\n.color, red, blue, green",
            "@case\n.color, red, blue\n.color green",
            "@case\n.color*\n red\n blue\n green",
            "@case\n.color; red; blue; green",
            "@case\n.color; red  ;  blue  ;  green",
            "@case\n.color, red\n blue\n green",
            "@case\n.color, red,,\n ,,blue\n green",
        ];

        let join_lines = vec![
            "@case\n.doc In a hole in the ground there lived a hobbit.",
            "@case\n.doc> In a hole\n in the ground\n there lived a hobbit.",
            "@case\n.doc|\n In a hole in the ground there lived a hobbit.",
        ];

        run_tests(split_values);
        run_tests(join_lines);
    }
}
