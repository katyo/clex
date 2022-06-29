/// Keyword token
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Keyword {
    /// auto
    Auto,
    /// break
    Break,
    /// case
    Case,
    /// char
    Char,
    /// const
    Const,
    /// continue
    Continue,
    /// default
    Default,
    /// do
    Do,
    /// double
    Double,
    /// else
    Else,
    /// enum
    Enum,
    /// extern
    Extern,
    /// float
    Float,
    /// for
    For,
    /// goto
    Goto,
    /// if
    If,
    /// inline
    Inline,
    /// int
    Int,
    /// long
    Long,
    /// register
    Register,
    /// restrict
    Restrict,
    /// return
    Return,
    /// short
    Short,
    /// signed
    Signed,
    /// sizeof
    SizeOf,
    /// static
    Static,
    /// struct
    Struct,
    /// switch
    Switch,
    /// typedef
    TypeDef,
    /// union
    Union,
    /// unsigned
    Unsigned,
    /// void
    Void,
    /// volatile
    Volatile,
    /// while
    While,
    /// _Alignas
    AlignAs,
    /// _Alignof
    AlignOf,
    /// _Atomic
    Atomic,
    /// _Bool
    Bool,
    /// _Complex
    Complex,
    /// _Generic
    Generic,
    /// _Imaginary
    Imaginary,
    /// _Noreturn
    NoReturn,
    /// _Static_assert
    StaticAssert,
    /// _Thread_local
    ThreadLocal,
    /// __func__
    FuncName,
}

impl std::str::FromStr for Keyword {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        KEYWORDS.get(s).copied().ok_or(())
    }
}

static KEYWORDS: phf::Map<&'static str, Keyword> = phf::phf_map! {
    "auto" => Keyword::Auto,
    "break" => Keyword::Break,
    "case" => Keyword::Case,
    "char" => Keyword::Char,
    "const" => Keyword::Const,
    "continue" => Keyword::Continue,
    "default" => Keyword::Default,
    "do" => Keyword::Do,
    "double" => Keyword::Double,
    "else" => Keyword::Else,
    "enum" => Keyword::Enum,
    "extern" => Keyword::Extern,
    "float" => Keyword::Float,
    "for" => Keyword::For,
    "goto" => Keyword::Goto,
    "if" => Keyword::If,
    "inline" => Keyword::Inline,
    "int" => Keyword::Int,
    "long" => Keyword::Long,
    "register" => Keyword::Register,
    "restrict" => Keyword::Restrict,
    "return" => Keyword::Return,
    "short" => Keyword::Short,
    "signed" => Keyword::Signed,
    "sizeof" => Keyword::SizeOf,
    "static" => Keyword::Static,
    "struct" => Keyword::Struct,
    "switch" => Keyword::Switch,
    "typedef" => Keyword::TypeDef,
    "union" => Keyword::Union,
    "unsigned" => Keyword::Unsigned,
    "void" => Keyword::Void,
    "volatile" => Keyword::Volatile,
    "while" => Keyword::While,
    "_Alignas" => Keyword::AlignAs,
    "_Alignof" => Keyword::AlignOf,
    "_Atomic" => Keyword::Atomic,
    "_Bool" => Keyword::Bool,
    "_Complex" => Keyword::Complex,
    "_Generic" => Keyword::Generic,
    "_Imaginary" => Keyword::Imaginary,
    "_Noreturn" => Keyword::NoReturn,
    "_Static_assert" => Keyword::StaticAssert,
    "_Thread_local" => Keyword::ThreadLocal,
    "__func__" => Keyword::FuncName,
};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn keyword() {
        assert_eq!("auto".parse::<Keyword>(), Ok(Keyword::Auto));
        assert_eq!("extern".parse::<Keyword>(), Ok(Keyword::Extern));
        assert_eq!("do".parse::<Keyword>(), Ok(Keyword::Do));
        assert_eq!("__func__".parse::<Keyword>(), Ok(Keyword::FuncName));
    }

    #[test]
    fn not_a_keyword() {
        assert_eq!("123".parse::<Keyword>(), Err(()));
        assert_eq!("done".parse::<Keyword>(), Err(()));
    }
}
