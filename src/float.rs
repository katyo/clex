pub trait Float: Sized {
    fn from_str(text: &str) -> Option<Self>;
}

macro_rules! float_impl {
    ($($(#[$meta:meta])* $type:ty;)*) => {
        $(
            $(#[$meta])*
            impl Float for $type {
                fn from_str(text: &str) -> Option<Self> {
                    text.parse().ok()
                }
            }
        )*
    };
}

float_impl! {
    f32;
    f64;
}

fn is_float_suffix(c: char) -> bool {
    matches!(c, 'f' | 'F' | 'l' | 'L')
}

pub fn extract<T: Float>(text: &str) -> Option<T> {
    T::from_str(text.trim_end_matches(is_float_suffix))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn not_a() {
        assert!(extract::<f32>("a").is_none());
        assert!(extract::<f64>("").is_none());
        assert!(extract::<f32>("-").is_none());
    }

    #[test]
    fn simple() {
        assert_eq!(extract::<f32>("1.25"), Some(1.25));
        assert_eq!(extract::<f64>("1.25"), Some(1.25));
    }

    #[test]
    fn suffixed() {
        assert_eq!(extract::<f32>("1.25f"), Some(1.25));
        assert_eq!(extract::<f64>("1.25L"), Some(1.25));
    }

    #[test]
    fn exponent() {
        assert_eq!(extract::<f32>("1.25e4"), Some(1.25e4));
        assert_eq!(extract::<f32>("1.25E+4"), Some(1.25e4));
        assert_eq!(extract::<f64>("1.25e-4"), Some(1.25e-4));
    }

    #[ignore]
    #[test]
    fn hexadecimal() {
        assert_eq!(extract::<f32>("0x1.4p3"), Some(10.0));
    }
}
