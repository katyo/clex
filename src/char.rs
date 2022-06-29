pub fn extract(text: &str) -> Option<char> {
    let text = text.trim_end();

    let text = if let Some(text) = text.strip_prefix('L') {
        // wide char literal
        text
    } else if let Some(text) = text.strip_prefix('u') {
        // UTF-16 char literals
        text
    } else {
        // UTF-32 char literal
        text.strip_prefix('U').unwrap_or(text)
    };

    if text.starts_with('\'') && text.ends_with('\'') {
        let text = &text[1..text.len() - 1];

        #[derive(Clone, Copy)]
        enum State {
            // In character
            Chr,
            // Escape sequence
            Esc,
            // Oct char (code, left)
            Oct(u8, u8),
            // Hex char (code, left, width)
            Hex(u32, u8, u8),
        }

        let mut out = None;
        let mut state = State::Chr;

        for chr in text.chars() {
            loop {
                if out.is_some() {
                    return None;
                }
                match state {
                    State::Chr => match chr {
                        '\\' => {
                            state = State::Esc;
                            break;
                        }
                        _ => out = Some(chr),
                    },
                    State::Esc => {
                        out = Some(match chr {
                            'a' => '\x07',
                            'b' => '\x08',
                            'v' => '\x0b',
                            'f' => '\x0c',
                            'n' => '\n',
                            'r' => '\r',
                            't' => '\t',
                            'e' => '\x1b',
                            'u' => {
                                state = State::Hex(0, 4, 4);
                                break;
                            }
                            'U' => {
                                state = State::Hex(0, 8, 8);
                                break;
                            }
                            'x' => {
                                state = State::Hex(0, 2, 2);
                                break;
                            }
                            '0'..='7' => {
                                state = State::Oct(chr as u8 - b'0', 2);
                                break;
                            }
                            _ => chr,
                        })
                    }
                    State::Hex(r, n, w) => {
                        if n > 0 {
                            state = State::Hex(
                                (r << 4)
                                    | match chr {
                                        '0'..='9' => chr as u32 - b'0' as u32,
                                        'a'..='f' => chr as u32 - (b'a' - 10) as u32,
                                        'A'..='F' => chr as u32 - (b'A' - 10) as u32,
                                        _ => {
                                            out = Some(char::from_u32(r).unwrap_or('\0'));
                                            state = State::Chr;
                                            continue;
                                        }
                                    },
                                n - 1,
                                w,
                            );
                            break;
                        } else {
                            out = Some(char::from_u32(r).unwrap_or('\0'));
                            state = State::Chr;
                            continue;
                        }
                    }
                    State::Oct(r, n) => {
                        if n > 0 {
                            state = State::Oct(
                                (r << 3)
                                    | match chr {
                                        '0'..='7' => chr as u8 - b'0',
                                        _ => {
                                            out = Some(r as _);
                                            state = State::Chr;
                                            continue;
                                        }
                                    },
                                n - 1,
                            );
                            break;
                        } else {
                            out = Some(r as _);
                            state = State::Chr;
                            continue;
                        }
                    }
                }
                state = State::Chr;
                break;
            }
        }

        match state {
            State::Hex(r, n, w) => {
                out = if n < w {
                    Some(char::from_u32(r).unwrap_or('\0'))
                } else {
                    None
                }
            }
            State::Oct(r, _) => out = Some(r as _),
            State::Esc => out = None,
            _ => (),
        }

        out
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn not_a() {
        assert!(extract("a").is_none());
        assert!(extract("'a").is_none());
        assert!(extract("a'").is_none());
        assert!(extract("'ab'").is_none());
        assert!(extract(r#"'\'"#).is_none());
        assert!(extract(r#"'\x'"#).is_none());
        assert!(extract(r#"'\u'"#).is_none());
        assert!(extract(r#"'\U'"#).is_none());
    }

    #[test]
    fn simple() {
        assert_eq!(extract("'a'").unwrap(), 'a');
        assert_eq!(extract("'a'\r\n").unwrap(), 'a');
    }

    #[test]
    fn escaped_slash() {
        assert_eq!(extract(r#"'\\'"#).unwrap(), '\\');
    }

    #[test]
    fn escaped_newline() {
        assert_eq!(extract(r#"'\n'"#).unwrap(), '\n');
    }

    #[test]
    fn escaped_escape() {
        assert_eq!(extract(r#"'\e'"#).unwrap(), '\x1b');
    }

    #[test]
    fn escaped_null() {
        assert_eq!(extract(r#"'\0'"#).unwrap(), '\0');
    }

    #[test]
    fn escaped_oct() {
        assert_eq!(extract(r#"'\01'"#).unwrap(), '\x01');
        assert_eq!(extract(r#"'\012'"#).unwrap(), '\n');
        assert_eq!(extract(r#"'\007'"#).unwrap(), '\x07');
    }

    #[test]
    fn escaped_hex() {
        assert_eq!(extract(r#"'\x0'"#).unwrap(), '\0');
        assert_eq!(extract(r#"'\xa'"#).unwrap(), '\x0a');
        assert_eq!(extract(r#"'\xA1'"#).unwrap(), '\u{a1}');
        assert_eq!(extract(r#"'\xa0'"#).unwrap(), '\u{a0}');
    }

    #[test]
    fn escaped_uni() {
        assert_eq!(extract(r#"'\u0'"#).unwrap(), '\0');
        assert_eq!(extract(r#"'\ua'"#).unwrap(), '\x0a');
        assert_eq!(extract(r#"'\uA1'"#).unwrap(), '\u{a1}');
        assert_eq!(extract(r#"'\uabc7'"#).unwrap(), '\u{abc7}');
        assert_eq!(extract(r#"'\Uabc70'"#).unwrap(), '\u{abc70}');
        assert_eq!(extract(r#"'\U10ffff'"#).unwrap(), '\u{10ffff}');
    }
}
