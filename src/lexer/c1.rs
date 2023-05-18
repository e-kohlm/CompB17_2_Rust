use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum C1Token {
    // Variants and their token/regex
    //Key Words
    #[token("bool")]
    KwBoolean,
    #[token("do")]
    KwDo,
    #[token("else")]
    KwElse,
    #[token("float")]
    KwFloat,
    #[token("for")]
    KwFor,
    #[token("if")]
    KwIf,
    #[token("int")]
    KwInt,
    #[token("printf")]
    KwPrintf,
    #[token("return")]
    KwReturn,
    #[token("void")]
    KwVoid,
    #[token("while")]
    KwWhile,

    //Operators
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Asterisk,
    #[token("/")]
    Slash,
    #[token("=")]
    Assign,
    #[token("==")]
    Eq,
    #[token("!=")]
    Neq,
    #[token("<")]
    Lss,
    #[token(">")]
    Grt,
    #[token("<=")]
    Leq,
    #[token(">=")]
    Geq,
    #[token("&&")]
    And,
    #[token("||")]
    Or,

    //Other
    #[token(",")]
    Comma,
    #[token(";")]
    Semicolon,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,

    //Term variants
    #[regex(r"\d+")]
    ConstInt,
    #[regex(r"\d*(\.\d+)?([eE][-+]?\d+)?")]
    ConstFloat,
    #[token("true")]
    #[token("false")]
    ConstBoolean,
    #[regex(r#""([^"\\]|\\.)*""#)]
    ConstString,
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Id,

    //Whitespaces, EOF, Tabs, and Comments to be ignored
    #[regex(r" ", logos::skip)]
    #[regex(r"\n", logos::skip)]
    #[regex(r"\t", logos::skip)]
    #[regex(r"//.*", logos::skip)]
    #[regex(r"/\*([^*]|\*[^/])*\*/", logos::skip)]
    Ignored,

    // Logos requires one token variant to handle errors
    #[error]
    Error,
}
