#[derive(Clone, Copy, Default)]
pub struct State {
    pub is_directive: bool,
    pub is_include: bool,
}

#[derive(Debug, Clone, Copy, logos::Logos, PartialEq, Eq)]
#[logos(extras = State)]
// octal digit
#[logos(subpattern o = "[0-7]")]
// decimal digit
#[logos(subpattern d = "[0-9]")]
// non-zero decimal digit
#[logos(subpattern nz = "[1-9]")]
// hexadecimal digit
#[logos(subpattern h = "[a-fA-F0-9]")]
// hexadecimal prefix
#[logos(subpattern hp = "0[xX]")]
// hexadecimal digit
#[logos(subpattern b = "[01]")]
// hexadecimal prefix
#[logos(subpattern bp = "0[bB]")]
// exponent
#[logos(subpattern e = "[eE][+-]?(?&d)+")]
#[logos(subpattern p = "[pP][+-]?(?&d)+")]
// float suffix
#[logos(subpattern fs = "[fFlL]")]
// integer suffix
#[logos(subpattern is = "([uU]([lL]|ll|LL)?)|(([lL]|ll|LL)[uU]?)")]
#[logos(subpattern l = "[a-zA-Z_$]")]
#[logos(subpattern a = "[a-zA-Z_$0-9]")]
// char prefix
#[logos(subpattern cp = r"[uUL]")]
// string prefix
#[logos(subpattern sp = r"u8|(?&cp)")]
// white space
#[logos(subpattern ws = r"[ \t\v\r\n\f]")]
// escape sequence
#[logos(subpattern es = r#"[\\](['"%?\\abefnrtv]|[0-7]+|[xu][a-fA-F0-9]+|[\r]?[\n])"#)]
pub enum Token {
    #[regex("//[^\r\n]*")]
    #[token("/*", |lex| {
        lex.bump(lex.remainder().find("*/")? + 2);
        Some(())
    })]
    Comment,

    #[regex(r"\.\.\.")]
    #[regex(r">>=|<<=|[+]=|-=|[*]=|/=|%=|&=|[\^]=|\|=")]
    #[regex(r">>|<<|[+][+]|--|->|&&|[|][|]|<=|>=|==|!=|<%|%>|<:|:>")]
    #[regex(r"[;{},:=()\[\].&!~\-+*/%<>^|?\\#]")]
    /*#[token("#", |lex| {
        lex.extras.is_directive = true;
        Some(())
    })]*/
    Symbol,

    #[regex(r"(?&cp)?'([^'\\\n]|(?&es))*'")]
    Char,

    #[regex(r#"((?&sp)?"([^"\\\n]|(?&es))*"(?&ws)*)+"#)]
    String,

    #[regex("((?&hp)(?&h)+|(?&bp)(?&b)+|(?&nz)(?&d)*|0(?&o)*)(?&is)?")]
    Int,

    #[regex(
        "((?&d)+(?&e)|(?&d)*[.](?&d)+(?&e)?|(?&d)+[.](?&e)?|(?&hp)((?&h)+(?&p)|(?&h)*[.](?&h)+(?&p)|(?&h)+[.](?&p)))(?&fs)?"
    )]
    Float,

    #[regex("(?&l)(?&a)*")]
    Identifier,

    #[error]
    #[regex(r"(?&ws)+", logos::skip)]
    Unknown,
}
