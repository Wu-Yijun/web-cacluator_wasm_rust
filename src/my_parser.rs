use std::f64;

use wasm_bindgen::prelude::*;

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

#[derive(Debug, Clone, Default)]
struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<Literal>,
    // [line, column]
    line_colum: [usize; 2],
    // [start, end]
    pos: [usize; 2],
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
enum TokenType {
    // -------- Single-character tokens --------
    /// (
    LeftParen,
    /// )
    RightParen,
    /// {
    LeftBrace,
    /// }
    RightBrace,
    /// [
    LeftSquare,
    /// ]
    RightSquare,
    /// ,
    Comma,
    /// .
    Dot,
    /// ^
    Caret,
    /// ;
    Semicolon,
    /// /
    Slash,
    /// *
    Star,
    /// &
    And,
    /// |
    Or,

    // -------- One or two character tokens --------
    /// !
    Bang,
    /// !=
    BangEqual,
    /// =
    Equal,
    /// ==
    EqualEqual,
    /// >
    Greater,
    /// >=
    GreaterEqual,
    /// <
    Less,
    /// <=
    LessEqual,
    /// -
    Minus,
    /// --
    MinusMinus,
    /// +
    Plus,
    /// ++
    PlusPlus,

    // -------- Literals --------
    /// [a-zA-Z_][a-zA-Z_0-9]*, not reserved words
    Identifier,
    /// [']([^'\\]|(\\.))[']
    Char,
    /// ["]([^"\\]|(\\.))*["]
    String,
    /// [0-9][0-9a-zA-Z_]*([\.][0-9])?([eE][+-]?[0-9])([0-9a-zA-Z_]*)
    /// 0x12_34_56_78_u32
    /// 12_345.678_910
    ///
    /// CANNOT *START* with **'.'**
    Number,
    /// true or false
    Bool,

    // -------- Keywords --------

    // Struct,
    // Else,
    // False,
    // Fun,
    // For,
    // If,
    // Nil,
    // Print,
    // Return,
    // Super,
    // This,
    // True,
    // Var,
    // While,

    // -------- skipped --------
    /// [ \t\n\r]
    Whitespace,
    /// [\n\r]
    NewLine,
    /// [\/][\*](.|\n)*[\*][\/]
    MuitiLineComment,
    /// [\/][\/][^\n]*
    SingleLineComment,

    // End of file
    #[default]
    EOF,
}

#[derive(Debug, Clone)]
enum Literal {
    Identifier(Identifier),
    Char(char),
    String(String),
    Number(f64),
    Bool(bool),
}

#[derive(Debug, Clone)]
struct Identifier {
    name: String,
    fun: bool,
    var: bool,
}

impl Parser {
    pub fn new_inline(line: String) -> Self {
        let tokens = vec![];
        let current = 0;
        let mut parser = Parser { tokens, current };
        parser.parse_line(line);
        parser
    }
    pub fn parse_line(&mut self, line: String) {
        let utf8_slice: Vec<char> = line.chars().collect();
        let line = 0;
        let mut colum = 0;
        let mut offset = 0;
        loop {
            let t = Token::from(&utf8_slice, [line, colum], offset);
            if t.is_eof() {
                break;
            }
            colum = t.line_colum[1] + t.pos[1] - t.pos[0];
            offset = t.pos[1];
            self.tokens.push(t);
        }
    }

    pub fn print(&self) {
        for t in self.tokens.iter() {
            t.print();
        }
    }
}

impl Token {
    fn from(text: &[char], mut line_colum: [usize; 2], mut offset: usize) -> Self {
        let mut len = 0;
        // skip whitespaces
        while offset < text.len() && TokenType::Whitespace.is_char(text[offset]) {
            if TokenType::NewLine.is_char(text[offset]) {
                line_colum[0] += 1;
                line_colum[1] = 0;
            } else {
                line_colum[1] += 1;
            }
            offset += 1;
        }
        // judge EOF
        if offset >= text.len() {
            return Token {
                token_type: TokenType::EOF,
                lexeme: "".to_string(),
                literal: None,
                line_colum,
                pos: [offset, offset + len],
            };
        }
        // judge Comments
        if TokenType::Slash.is_char(text[offset]) && offset + 1 < text.len() {
            if TokenType::Slash.is_char(text[offset + 1]) {
                // single line comment
                len += 2;
                while offset + len < text.len() && !TokenType::NewLine.is_char(text[offset + len]) {
                    len += 1;
                }
                return Token {
                    token_type: TokenType::SingleLineComment,
                    lexeme: text[offset..offset + len].iter().collect(),
                    literal: None,
                    line_colum,
                    pos: [offset, offset + len],
                };
            } else if TokenType::Star.is_char(text[offset + 1]) {
                // multi line comment
                if offset + 3 >= text.len() {
                    // not enough chars before eof
                    return Token {
                        token_type: TokenType::MuitiLineComment,
                        lexeme: text[offset..].iter().collect(),
                        literal: None,
                        line_colum,
                        pos: [offset, text.len()],
                    };
                }
                len += 3;
                while offset + len < text.len()
                    && !(text[offset + len - 1] == '*' && text[offset + len] == '/')
                {
                    len += 1;
                }
                return Token {
                    token_type: TokenType::MuitiLineComment,
                    lexeme: text[offset..text.len().min(offset + len + 1)]
                        .iter()
                        .collect(),
                    literal: None,
                    line_colum,
                    pos: [offset, text.len().min(offset + len + 1)],
                };
            }
        }

        match text[offset] {
            '\'' if Some(&'\\') != text.get(offset + 1) && Some(&'\'') == text.get(offset + 2) => {
                // char '.'
                Token {
                    token_type: TokenType::Char,
                    lexeme: text[offset..offset + 3].iter().collect(),
                    literal: Some(Literal::from_char(text[offset + 1])),
                    line_colum,
                    pos: [offset, offset + 3],
                }
            }
            '\'' if Some(&'\\') == text.get(offset + 1) && Some(&'\'') == text.get(offset + 3) => {
                // char '\.'
                Token {
                    token_type: TokenType::Char,
                    lexeme: text[offset..offset + 4].iter().collect(),
                    literal: Some(Literal::from_slash_char(text[offset + 2])),
                    line_colum,
                    pos: [offset, offset + 4],
                }
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                // Identifier
                len = 1;
                while offset + len < text.len() && TokenType::Identifier.is_char(text[offset + len])
                {
                    len += 1;
                }
                let lexeme: String = text[offset..offset + len].iter().collect();
                let literal = Some(Literal::Identifier(Identifier::new(lexeme.clone())));
                Token {
                    token_type: TokenType::Identifier,
                    lexeme,
                    literal,
                    line_colum,
                    pos: [offset, offset + len],
                }
            }
            '0'..='9' => {
                len += 1;
                while offset + len < text.len() && TokenType::Number.is_char(text[offset + len]) {
                    len += 1;
                }
                if Some(&'.') == text.get(offset + len)
                    && text
                        .get(offset + len + 1)
                        .is_some_and(|c| c.is_ascii_digit())
                {
                    // with fractional part
                    len += 1;
                    while offset + len < text.len() && TokenType::Number.is_char(text[offset + len])
                    {
                        len += 1;
                    }
                }
                if (text[offset + len - 1] == 'E' || text[offset + len - 1] == 'e')
                    && (text[offset + len] == '+' || text[offset + len] == '-')
                {
                    // with expontional part
                    len += 1;
                    while offset + len < text.len() && TokenType::Number.is_char(text[offset + len])
                    {
                        len += 1;
                    }
                }
                let lexeme: String = text[offset..offset + len].iter().collect();
                let literal = Some(Literal::from_number(&lexeme));

                return Token {
                    token_type: TokenType::Number,
                    lexeme,
                    literal,
                    line_colum,
                    pos: [offset, offset + len],
                };
            }
            _ => {
                if let Some(t) = TokenType::from_char(text[offset]) {
                    len = 1;
                    let token_type = match t {
                        TokenType::Plus if Some(&'+') == text.get(offset + 1) => {
                            len = 2;
                            TokenType::PlusPlus
                        }
                        TokenType::Minus if Some(&'-') == text.get(offset + 1) => {
                            len = 2;
                            TokenType::MinusMinus
                        }
                        TokenType::Bang if Some(&'=') == text.get(offset + 1) => {
                            len = 2;
                            TokenType::BangEqual
                        }
                        TokenType::Equal if Some(&'=') == text.get(offset + 1) => {
                            len = 2;
                            TokenType::EqualEqual
                        }
                        TokenType::Greater if Some(&'=') == text.get(offset + 1) => {
                            len = 2;
                            TokenType::GreaterEqual
                        }
                        TokenType::Less if Some(&'=') == text.get(offset + 1) => {
                            len = 2;
                            TokenType::EqualEqual
                        }
                        TokenType::LessEqual if Some(&'=') == text.get(offset + 1) => {
                            len = 2;
                            TokenType::EqualEqual
                        }
                        others => others,
                    };
                    Token {
                        token_type,
                        lexeme: text[offset..offset + len].iter().collect(),
                        literal: None,
                        line_colum,
                        pos: [offset, offset + len],
                    }
                } else {
                    Token::default()
                }
            }
        }
    }

    fn is_eof(&self) -> bool {
        self.token_type == TokenType::EOF
    }

    fn print(&self) {
        print!("{:?} ", self.token_type);
    }
}

impl TokenType {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '(' => Some(TokenType::LeftParen),
            ')' => Some(TokenType::RightParen),
            '{' => Some(TokenType::LeftBrace),
            '}' => Some(TokenType::LeftBrace),
            '[' => Some(TokenType::LeftSquare),
            ']' => Some(TokenType::RightSquare),
            ',' => Some(TokenType::Comma),
            '.' => Some(TokenType::Dot),
            '^' => Some(TokenType::Caret),
            '-' => Some(TokenType::Minus),
            '+' => Some(TokenType::Plus),
            ';' => Some(TokenType::Semicolon),
            '/' => Some(TokenType::Slash),
            '*' => Some(TokenType::Star),
            '&' => Some(TokenType::And),
            '|' => Some(TokenType::Or),
            '!' => Some(TokenType::Bang),
            '=' => Some(TokenType::Equal),
            '>' => Some(TokenType::Greater),
            '<' => Some(TokenType::LessEqual),
            _ => None,
        }
    }
    pub fn is_char(&self, c: char) -> bool {
        match self {
            TokenType::LeftParen => c == '(',
            TokenType::RightParen => c == ')',
            TokenType::LeftBrace => c == '{',
            TokenType::RightBrace => c == '}',
            TokenType::LeftSquare => c == '[',
            TokenType::RightSquare => c == ']',
            TokenType::Comma => c == ',',
            TokenType::Dot => c == '.',
            TokenType::Caret => c == '^',
            TokenType::Minus => c == '-',
            TokenType::Plus => c == '+',
            TokenType::Semicolon => c == ';',
            TokenType::Slash => c == '/',
            TokenType::Star => c == '*',
            TokenType::Bang => c == '!',
            TokenType::Equal => c == '=',
            TokenType::Greater => c == '>',
            TokenType::Less => c == '<',
            TokenType::Whitespace => c == ' ' || c == '\t' || c == '\n' || c == '\r',
            TokenType::NewLine => c == '\n' || c == '\r',
            TokenType::Identifier => c.is_ascii_alphanumeric() || c == '_',
            TokenType::Number => c.is_ascii_alphanumeric() || c == '_',
            _ => false,
        }
    }
}

impl Literal {
    pub fn from_char(c: char) -> Self {
        Literal::Char(c)
    }
    pub fn from_slash_char(c: char) -> Self {
        match c {
            '\\' => Literal::Char('\\'),
            '\'' => Literal::Char('\''),
            '"' => Literal::Char('"'),
            'n' => Literal::Char('\n'),
            'r' => Literal::Char('\r'),
            't' => Literal::Char('\t'),
            _ => Literal::Char(c),
        }
    }
    pub fn from_number(n: &String) -> Self {
        println!("num: {n}");
        let mut n = n.replace('_', "").to_lowercase();
        let mut radix = 10;
        let mut integer = false;
        let mut error = false;
        let mut expo = 0;
        if n.len() > 2 {
            match &n[0..2] {
                "0x" => {
                    n = n.split_off(2);
                    radix = 16;
                    integer = true;
                }
                "0b" => {
                    n = n.split_off(2);
                    radix = 2;
                    integer = true;
                }
                "0o" => {
                    n = n.split_off(2);
                    radix = 8;
                    integer = true;
                }
                _ => {}
            }
        }
        if n.len() > 3 {
            match &n[n.len() - 3..n.len()] {
                "i32" | "i64" | "i16" | "i8" => {
                    let _ = n.split_off(n.len() - 3);
                    integer = true;
                }
                "u32" | "u64" | "u16" | "u8" => {
                    let _ = n.split_off(n.len() - 3);
                    integer = true;
                }
                "f32" | "f64" => {
                    let _ = n.split_off(n.len() - 3);
                    if integer {
                        error = true;
                    }
                }
                _ => {}
            }
        }
        if integer && n.contains('.') {
            error = true;
        }
        if n.contains('e') {
            if integer {
                if radix != 16 {
                    error = true;
                }
            } else {
                let ex = n.split_off(n.find('e').unwrap());
                expo = if let Ok(res) = ex[1..].parse::<i32>() {
                    res
                } else {
                    error = true;
                    0
                };
            }
        }
        let d = if integer {
            if let Ok(res) = u64::from_str_radix(&n, radix) {
                res as f64
            } else {
                error = true;
                0.0
            }
        } else {
            if let Ok(res) = n.parse::<f64>() {
                res * 10.0f64.powi(expo)
            } else {
                error = true;
                0.0f64
            }
        };
        if error {
            println!("Number parse Error!");
        }
        Literal::Number(d)
    }
}

impl Identifier {
    fn new(name: String) -> Self {
        Identifier {
            name,
            fun: false,
            var: false,
        }
    }
}
