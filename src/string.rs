pub fn extract(text: &str) -> Option<String> {
    let text = text.trim_end();

    let text = if let Some(text) = text.strip_prefix('L') {
        // wide string literal
        text
    } else if let Some(text) = text.strip_prefix('u') {
        // UTF-8 or UTF-16 string literals
        text.strip_prefix('8').unwrap_or(text)
    } else {
        // UTF-32 string literal
        text.strip_prefix('U').unwrap_or(text)
    };

    if text.starts_with('"') && text.ends_with('"') {
        let text = &text[1..text.len() - 1];

        let mut out = String::with_capacity(text.len());

        #[derive(Clone, Copy)]
        enum State {
            // In string
            Str,
            // Escape sequence
            Esc,
            // Oct char (code, left)
            Oct(u8, u8),
            // Hex char (code, left, width)
            Hex(u32, u8, u8),
            // out of string
            Out,
        }

        let state = text.chars().fold(State::Str, |mut state, mut chr| {
            loop {
                out.push(match state {
                    State::Str => match chr {
                        '\\' => return State::Esc,
                        '"' => return State::Out,
                        _ => chr,
                    },
                    State::Out => match chr {
                        '"' => return State::Str,
                        _ => return State::Out,
                    },
                    State::Esc => match chr {
                        'a' => '\x07',
                        'b' => '\x08',
                        'v' => '\x0b',
                        'f' => '\x0c',
                        'n' => '\n',
                        'r' => '\r',
                        't' => '\t',
                        'e' => '\x1b',
                        'u' => return State::Hex(0, 4, 4),
                        'U' => return State::Hex(0, 8, 8),
                        'x' => return State::Hex(0, 2, 2),
                        '0'..='7' => return State::Oct(chr as u8 - b'0', 2),
                        //'\'' | '"' | '?' | '\\' => chr,
                        oth => oth,
                    },
                    State::Hex(r, n, w) => {
                        if n > 0 {
                            return State::Hex(
                                (r << 4)
                                    | match chr {
                                        '0'..='9' => chr as u32 - b'0' as u32,
                                        'a'..='f' => chr as u32 - (b'a' - 10) as u32,
                                        'A'..='F' => chr as u32 - (b'A' - 10) as u32,
                                        oth => {
                                            out.push(char::from_u32(r).unwrap_or('\0'));
                                            state = State::Str;
                                            chr = oth;
                                            continue;
                                        }
                                    },
                                n - 1,
                                w,
                            );
                        }
                        out.push(char::from_u32(r).unwrap_or('\0'));
                        state = State::Str;
                        continue;
                    }
                    State::Oct(r, n) => {
                        if n > 0 {
                            return State::Oct(
                                (r << 3)
                                    | match chr {
                                        '0'..='7' => chr as u8 - b'0',
                                        oth => {
                                            out.push(r as _);
                                            state = State::Str;
                                            chr = oth;
                                            continue;
                                        }
                                    },
                                n - 1,
                            );
                        }
                        out.push(r as _);
                        state = State::Str;
                        continue;
                    }
                });
                break;
            }
            State::Str
        });

        match state {
            State::Hex(r, n, w) => {
                if n < w {
                    out.push(char::from_u32(r).unwrap_or('\0'));
                }
            }
            State::Oct(r, _) => out.push(r as _),
            _ => {}
        }

        Some(out)
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn not_a() {
        assert!(extract("abc de f").is_none());
        assert!(extract("\"abc de fg").is_none());
        assert!(extract("abc de fg\"").is_none());

        assert!(extract("abc \\").is_none());
        assert!(extract("abc\\x").is_none());
        assert!(extract("abc\\u").is_none());
        assert!(extract("abc\\U").is_none());
    }

    #[test]
    fn single() {
        assert_eq!(extract("\"abc de f\"").unwrap(), "abc de f");
        assert_eq!(extract("\"abc def\"\r\n ").unwrap(), "abc def");
    }

    #[test]
    fn multi() {
        assert_eq!(extract("\"ab\"\"c\"").unwrap(), "abc");
        assert_eq!(extract("\"ab\" \"c\"\t \"def\"").unwrap(), "abcdef");
        assert_eq!(extract("\"ab\"\r\n\"c\"\n ").unwrap(), "abc");
    }

    #[test]
    fn escaped_slash() {
        assert_eq!(extract(r#""abc \\de f""#).unwrap(), "abc \\de f");
    }

    #[test]
    fn escaped_newline() {
        assert_eq!(extract(r#""abc \nde f""#).unwrap(), "abc \nde f");
    }

    #[test]
    fn escaped_escape() {
        assert_eq!(extract(r#""abc \ede f""#).unwrap(), "abc \x1bde f");
    }

    #[test]
    fn escaped_null() {
        assert_eq!(extract(r#""abc \0de f""#).unwrap(), "abc \0de f");
        assert_eq!(extract(r#""abc \0 f""#).unwrap(), "abc \0 f");
    }

    #[test]
    fn escaped_oct() {
        assert_eq!(extract(r#""\01 de""#).unwrap(), "\x01 de");
        assert_eq!(extract(r#""\012f""#).unwrap(), "\nf");
        assert_eq!(extract(r#""\007 f""#).unwrap(), "\x07 f");
    }

    #[test]
    fn escaped_hex() {
        assert_eq!(extract(r#""\x0h ""#).unwrap(), "\0h ");
        assert_eq!(extract(r#"" \xa ""#).unwrap(), " \x0a ");
        assert_eq!(extract(r#""\xA1 ""#).unwrap(), "\u{a1} ");
        assert_eq!(extract(r#"" \xa00 ""#).unwrap(), " \u{a0}0 ");
        assert_eq!(extract(r#"" \xa""#).unwrap(), " \x0a");
    }

    #[test]
    fn escaped_uni() {
        assert_eq!(extract(r#"" \u0h ""#).unwrap(), " \0h ");
        assert_eq!(extract(r#""\ua ""#).unwrap(), "\x0a ");
        assert_eq!(extract(r#""\ua""#).unwrap(), "\x0a");
        assert_eq!(extract(r#""\uA1 ""#).unwrap(), "\u{a1} ");
        assert_eq!(extract(r#""\uabc70 ""#).unwrap(), "\u{abc7}0 ");
        assert_eq!(extract(r#""\uabc7""#).unwrap(), "\u{abc7}");
        assert_eq!(extract(r#""\Uabc70 ""#).unwrap(), "\u{abc70} ");
        assert_eq!(extract(r#""\U10ffff ""#).unwrap(), "\u{10ffff} ");
    }
}
