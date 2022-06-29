pub trait Int: Sized {
    fn int_from_str(text: &str, radix: u32) -> Option<Self>;
}

macro_rules! int_impl {
    ($($(#[$meta:meta])* $type:ty;)*) => {
        $(
            $(#[$meta])*
            impl Int for $type {
                fn int_from_str(text: &str, radix: u32) -> Option<Self> {
                    <$type>::from_str_radix(text, radix).ok()
                }
            }
        )*
    };
}

int_impl! {
    u8;
    i8;
    u16;
    i16;
    u32;
    i32;
    u64;
    i64;
    u128;
    i128;
    #[cfg(feature = "ethnum")]
    ethnum::u256;
    #[cfg(feature = "ethnum")]
    ethnum::i256;
}

fn is_int_suffix(c: char) -> bool {
    matches!(c, 'u' | 'U' | 'l' | 'L')
}

pub fn extract<T: Int>(text: &str) -> Option<T> {
    let text = text.trim_end_matches(is_int_suffix);
    let (text, radix) = text
        .strip_prefix("0x")
        .map(|text| (text, 16))
        .or_else(|| text.strip_prefix("0X").map(|text| (text, 16)))
        .or_else(|| text.strip_prefix("0b").map(|text| (text, 2)))
        .or_else(|| text.strip_prefix("0B").map(|text| (text, 2)))
        .or_else(|| {
            if text != "0" {
                text.strip_prefix('0').map(|text| (text, 8))
            } else {
                None
            }
        })
        .unwrap_or((text, 10));
    T::int_from_str(text, radix)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn not_a() {
        assert!(extract::<u8>("a").is_none());
        assert!(extract::<u16>("-").is_none());
        assert!(extract::<i32>("").is_none());
    }

    #[test]
    fn decimal() {
        assert_eq!(extract::<u8>("123"), Some(123));
        assert_eq!(extract::<i8>("123"), Some(123));
        assert_eq!(extract::<u32>("123"), Some(123));
        assert_eq!(extract::<i32>("123"), Some(123));
    }

    #[test]
    fn suffixed() {
        assert_eq!(extract::<u8>("12l"), Some(12));
        assert_eq!(extract::<u16>("12L"), Some(12));
        assert_eq!(extract::<i32>("12ll"), Some(12));
        assert_eq!(extract::<u128>("12ul"), Some(12));
        assert_eq!(extract::<u8>("12ull"), Some(12));
    }

    #[test]
    fn octal() {
        assert_eq!(extract::<i8>("07"), Some(7));
        assert_eq!(extract::<u8>("0127"), Some(0o127));
    }

    #[test]
    fn hexadecimal() {
        assert_eq!(extract::<i8>("0x0"), Some(0));
        assert_eq!(extract::<u16>("0X1a9b"), Some(0x1a9b));
    }

    #[test]
    fn binary() {
        assert_eq!(extract::<i8>("0b0"), Some(0));
        assert_eq!(extract::<u16>("0B1011"), Some(0b1011));
    }

    #[cfg(feature = "ethnum")]
    #[test]
    fn ethnum() {
        use ethnum::{AsI256, AsU256};

        assert_eq!(
            extract::<ethnum::u256>("0xffffff00000000000000000000000000"),
            Some(0xffffff00000000000000000000000000u128.as_u256())
        );
        assert_eq!(
            extract::<ethnum::i256>("0Xffffff00000000000000000000000000u"),
            Some(0xffffff00000000000000000000000000u128.as_i256())
        );
    }
}
