use std::f64;

use js_sys::Function;
use wasm_bindgen::prelude::*;

#[derive(Debug)]
pub struct LexicalParser {
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
    /// :
    Colon,
    /// ^
    Caret,
    /// ;
    Semicolon,
    /// /
    Slash,
    /// %
    Percent,
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
    /// [ \t\r]
    Whitespace,
    /// [\n]
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
    is_neg: bool,
}

impl LexicalParser {
    pub fn new_inline(line: String) -> Self {
        let tokens = vec![];
        let current = 0;
        let mut parser = LexicalParser { tokens, current };
        parser.parse_line(line);
        parser
    }
    pub fn parse_line(&mut self, line: String) {
        let utf8_slice: Vec<char> = line.chars().collect();
        let mut line = 0;
        let mut colum = 0;
        let mut offset = 0;
        loop {
            let t = Token::from(&utf8_slice, [line, colum], offset);
            if t.is_eof() {
                break;
            }
            colum = t.line_colum[1] + t.pos[1] - t.pos[0];
            offset = t.pos[1];
            if t.is_newline() {
                colum = 0;
                line += 1;
            }
            self.tokens.push(t);
        }
    }

    pub fn print(&self, level: usize) -> String {
        self.tokens.iter().map(|t| t.print(level)).collect()
    }

    pub fn parse(&self) -> Option<Expression> {
        Some(Expression::from(&self.tokens)?.0)
    }
}

impl Token {
    fn from(text: &[char], mut line_colum: [usize; 2], mut offset: usize) -> Self {
        let mut len = 0;
        // skip whitespaces
        while offset < text.len() && TokenType::Whitespace.is_char(text[offset]) {
            line_colum[1] += 1;
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
        // judge NewLine
        if TokenType::NewLine.is_char(text[offset]) {
            line_colum[0] += 1;
            return Token {
                token_type: TokenType::NewLine,
                lexeme: "\n".to_string(),
                literal: None,
                line_colum,
                pos: [offset, offset + 1],
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
        // judge Keywords
        match text[offset] {
            't' if char_starts_with(text, offset, "true") => {
                // text starts with true and text[4] is not alphanumeric
                // true
                return Token {
                    token_type: TokenType::Bool,
                    lexeme: text[offset..offset + 4].iter().collect(),
                    literal: Literal::from_bool(true),
                    line_colum,
                    pos: [offset, offset + 4],
                };
            }
            'f' if char_starts_with(text, offset, "false") => {
                // text starts with true and text[4] is not alphanumeric
                // false
                return Token {
                    token_type: TokenType::Bool,
                    lexeme: text[offset..offset + 5].iter().collect(),
                    literal: Literal::from_bool(false),
                    line_colum,
                    pos: [offset, offset + 5],
                };
            }
            _ => {}
        }
        // others mainly literals
        match text[offset] {
            '\'' if Some(&'\\') != text.get(offset + 1) && Some(&'\'') == text.get(offset + 2) => {
                // char '.'
                Token {
                    token_type: TokenType::Char,
                    lexeme: text[offset..offset + 3].iter().collect(),
                    literal: Literal::from_char(text[offset + 1]),
                    line_colum,
                    pos: [offset, offset + 3],
                }
            }
            '\'' if Some(&'\\') == text.get(offset + 1) && Some(&'\'') == text.get(offset + 3) => {
                // char '\.'
                Token {
                    token_type: TokenType::Char,
                    lexeme: text[offset..offset + 4].iter().collect(),
                    literal: Literal::from_slash_char(text[offset + 2]),
                    line_colum,
                    pos: [offset, offset + 4],
                }
            }
            '"' => {
                len += 1;
                let mut literal = vec![];
                let mut slash_flag = false;
                while offset + len < text.len()
                    && (slash_flag || text[offset + len] != '"')
                    && !TokenType::NewLine.is_char(text[offset + len])
                {
                    let c = text[offset + len];
                    if slash_flag {
                        literal.push(slash_char(c));
                        slash_flag = false;
                    } else if c == '\\' {
                        slash_flag = true;
                    } else {
                        literal.push(c);
                    }
                    len += 1;
                }
                let lexeme = text[offset..text.len().min(offset + len + 1)]
                    .iter()
                    .collect();
                Token {
                    token_type: TokenType::String,
                    lexeme,
                    literal: Literal::from_char_vec(literal),
                    line_colum,
                    pos: [offset, text.len().min(offset + len + 1)],
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
                let literal = Literal::from_number(&lexeme);

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
                            TokenType::LessEqual
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

    fn is_literal(&self) -> bool {
        match self.token_type {
            TokenType::Char | TokenType::String | TokenType::Number | TokenType::Bool => true,
            _ => false,
        }
    }
    fn is_identifier(&self) -> bool {
        self.token_type == TokenType::Identifier
    }
    fn is_pos_neg(&self) -> bool {
        self.token_type == TokenType::Plus || self.token_type == TokenType::Minus
    }
    fn is_eof(&self) -> bool {
        self.token_type == TokenType::EOF
    }
    fn is_calc_op(&self) -> bool {
        match self.token_type {
            TokenType::Plus
            | TokenType::Minus
            | TokenType::Star
            | TokenType::Slash
            | TokenType::Percent
            | TokenType::Caret => true,
            _ => false,
        }
    }
    fn is_newline(&self) -> bool {
        self.token_type == TokenType::NewLine
    }
    fn is_skipped(&self) -> bool {
        match self.token_type {
            TokenType::Whitespace
            | TokenType::NewLine
            | TokenType::MuitiLineComment
            | TokenType::SingleLineComment => true,
            _ => false,
        }
    }
    fn print(&self, level: usize) -> String {
        match level {
            5 => format!("{:#?}\n", self),
            4 => format!("{:?}\n", self),
            3 => {
                let t = format!("Type:{:<20?}", self.token_type);
                let t2 = format!(
                    "{}\t<Line, Colum>{:?} \t<Start, End>{:?} \tContent({:?}) \tLiteral({:?})\n",
                    " ".repeat(20 - t.len().min(20)),
                    self.line_colum,
                    self.pos,
                    self.lexeme,
                    self.literal
                );
                t + &t2
            }
            2 => format!(
                "{:?}{:?}({:?})\n",
                self.token_type, self.line_colum, self.lexeme
            ),
            1 => format!("{:?}({:?}) ", self.token_type, self.lexeme),
            _ => format!("{:?} ", self.token_type),
        }
    }
}

impl TokenType {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '(' => Some(TokenType::LeftParen),
            ')' => Some(TokenType::RightParen),
            '{' => Some(TokenType::LeftBrace),
            '}' => Some(TokenType::RightBrace),
            '[' => Some(TokenType::LeftSquare),
            ']' => Some(TokenType::RightSquare),
            ',' => Some(TokenType::Comma),
            '.' => Some(TokenType::Dot),
            '^' => Some(TokenType::Caret),
            '-' => Some(TokenType::Minus),
            '+' => Some(TokenType::Plus),
            ':' => Some(TokenType::Colon),
            ';' => Some(TokenType::Semicolon),
            '/' => Some(TokenType::Slash),
            '%' => Some(TokenType::Percent),
            '*' => Some(TokenType::Star),
            '&' => Some(TokenType::And),
            '|' => Some(TokenType::Or),
            '!' => Some(TokenType::Bang),
            '=' => Some(TokenType::Equal),
            '>' => Some(TokenType::Greater),
            '<' => Some(TokenType::Less),
            _ => None,
        }
    }
    pub fn to_char(&self) -> char {
        match self {
            TokenType::LeftParen => '(',
            TokenType::RightParen => ')',
            TokenType::LeftBrace => '{',
            TokenType::RightBrace => '}',
            TokenType::LeftSquare => '[',
            TokenType::RightSquare => ']',
            TokenType::Comma => ',',
            TokenType::Dot => '.',
            TokenType::Caret => '^',
            TokenType::Minus => '-',
            TokenType::Plus => '+',
            TokenType::Semicolon => ';',
            TokenType::Colon => ':',
            TokenType::Slash => '/',
            TokenType::Percent => '%',
            TokenType::Star => '*',
            TokenType::Bang => '!',
            TokenType::Equal => '=',
            TokenType::Greater => '>',
            TokenType::Less => '<',
            _ => '\0',
        }
    }
    pub fn is_char(&self, c: char) -> bool {
        match self {
            TokenType::Whitespace => c == ' ' || c == '\t' || c == '\r',
            TokenType::NewLine => c == '\n',
            TokenType::Identifier => c.is_ascii_alphanumeric() || c == '_',
            TokenType::Number => c.is_ascii_alphanumeric() || c == '_',
            _ => c == self.to_char(),
        }
    }
}

impl Literal {
    pub fn from_char(c: char) -> Option<Self> {
        Some(Literal::Char(c))
    }
    pub fn from_slash_char(c: char) -> Option<Self> {
        Some(Literal::Char(slash_char(c)))
    }
    pub fn from_number(n: &String) -> Option<Self> {
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
        Some(Literal::Number(d))
    }
    pub fn from_char_vec(cs: Vec<char>) -> Option<Self> {
        let s: String = cs.into_iter().collect();
        Some(Literal::String(s))
    }
    pub fn from_bool(b: bool) -> Option<Self> {
        Some(Literal::Bool(b))
    }

    pub fn into_neg(self, is_neg: bool) -> Option<Self> {
        if is_neg {
            match self {
                Literal::Bool(b) => Some(Literal::Bool(!b)),
                Literal::Number(d) => Some(Literal::Number(-d)),
                Literal::Identifier(mut x) => {
                    x.is_neg = !x.is_neg;
                    Some(Literal::Identifier(x))
                }
                _ => None,
            }
        } else {
            match self {
                Literal::Bool(..) | Literal::Number(..) | Literal::Identifier(..) => Some(self),
                _ => None,
            }
        }
    }

    pub fn print(&self, level: usize) -> String {
        if level < 3 {
            match self {
                Literal::Identifier(i) => i.name.clone(),
                Literal::Char(c) => c.to_string(),
                Literal::String(s) => s.to_owned(),
                Literal::Number(d) => d.to_string(),
                Literal::Bool(b) => b.to_string(),
            }
        } else if level < 10 {
            match self {
                Literal::Identifier(i) => format!("<{}>", i.name.clone()),
                Literal::Char(c) => format!("'{}'", c.to_string()),
                Literal::String(s) => format!("\"{}\"", s.to_owned()),
                Literal::Number(d) => format!("{}", d.to_string()),
                Literal::Bool(b) => format!("{}", b.to_string()),
            }
        } else {
            match self {
                Literal::Identifier(i) => {
                    format!("<span class='syntax_identifier'>{}</span>", i.name.clone())
                }
                Literal::Char(c) => format!("<span class='syntax_char'>'{}'</span>", c.to_string()),
                Literal::String(s) => {
                    format!("<span class='syntax_string'>\"{}\"</span>", s.to_owned())
                }
                Literal::Number(d) => {
                    format!("<span class='syntax_number'>{}</span>", d.to_string())
                }
                Literal::Bool(b) => format!("<span class='syntax_bool'>{}</span>", b.to_string()),
            }
        }
    }
}

impl Identifier {
    fn new(name: String) -> Self {
        Identifier {
            name,
            fun: false,
            var: false,
            is_neg: false,
        }
    }
}

fn slash_char(c: char) -> char {
    match c {
        '\\' => '\\',
        '\'' => '\'',
        '"' => '"',
        'n' => '\n',
        'r' => '\r',
        't' => '\t',
        _ => c,
    }
}

fn char_starts_with(c: &[char], offset: usize, s: &str) -> bool {
    if c.len() < s.len() + offset
        || c.get(offset + s.len())
            .is_some_and(|c| c.is_ascii_alphanumeric())
    {
        return false;
    }
    let mut i = 0;
    for c0 in s.chars() {
        if c0 != c[offset + i] {
            return false;
        }
        i += 1;
    }
    true
}

// ----------- syntax parser ------------------- //

#[derive(Debug, Clone)]
pub enum Expression {
    /// exp ([+-*/] exp)*
    Operation(CalcUnit, Vec<(TokenType, CalcUnit)>),
}

#[derive(Debug, Clone)]
enum CalcUnit {
    /// 123
    Literal(Literal),
    /// - 123
    NegVal(Literal),
    /// x
    Identifier(Identifier),
    /// - 123
    NegVar(Identifier),
    /// f(...)
    Function(Identifier, Tuple),
    /// (...)
    Tuple(Tuple),
}

#[derive(Debug, Clone)]
struct Tuple {
    val: Vec<Expression>,
}

impl Expression {
    pub fn from(tks: &[Token]) -> Option<(Self, usize)> {
        let mut offset = 0;
        // 0 -> 1 : CalcUnit
        // 1 -> 2 : Token.is_calc_op()
        // 1 -> 1 : CalcUnit // default multiply
        // 1 -> Ok: Else
        // 2 -> 1 : CalcUnit
        let mut state = 0;
        let mut unit = None;
        let mut units = vec![];
        let mut op = TokenType::Star;
        while offset < tks.len() && !tks[offset].is_eof() {
            if tks[offset].is_skipped() {
                offset += 1;
                continue;
            }
            match state {
                1 if tks[offset].is_calc_op() => {
                    op = tks[offset].token_type;
                    offset += 1;
                    state = 2;
                    continue;
                }
                0 | 1 | 2 => {
                    if let Some((cu, len)) = CalcUnit::from(&tks[offset..]) {
                        offset += len;
                        if state == 0 {
                            unit = Some(cu);
                        } else {
                            units.push((op, cu));
                            op = TokenType::Star;
                        }
                        state = 1;
                        continue;
                    } else if state == 1 {
                        return Some((Expression::Operation(unit?, units), offset));
                    }
                }
                _ => {}
            }
            return None;
        }
        Some((Expression::Operation(unit?, units), offset))
    }

    // level 11: with html
    pub fn print(&self, level: usize) -> String {
        if level == 11 {
            match self {
                Expression::Operation(cu, us) => {
                    let mut res = cu.print(level);
                    for (tt, u) in us {
                        res += &format!(
                            " <span class='syntax_operator'>{}</span> {}",
                            tt.to_char(),
                            u.print(level)
                        );
                    }
                    format!("<span class='syntax_expression'>{}</span>", res)
                }
            }
        } else {
            match self {
                Expression::Operation(cu, us) => {
                    let mut res = cu.print(level);
                    for (tt, u) in us {
                        res += &format!(" {} {}", tt.to_char(), u.print(level));
                    }
                    format!("{}", res)
                }
            }
        }
    }

    pub fn tree(&self, level: usize, html: bool) -> String {
        match self {
            Expression::Operation(cu, us) => {
                let mut res = "+Expression: \n".to_string();
                res += &(INDENT.repeat(level) + "+---" + &cu.tree(level + 1, html));
                for (tt, u) in us {
                    res += "\n";
                    res += &(INDENT.repeat(level)
                        + "| Operator "
                        + &tree_node(html, &tt.to_char().to_string()));
                    res += "\n";
                    res += &(INDENT.repeat(level) + "+---" + &u.tree(level + 1, html));
                }
                res
            }
        }
    }
}

impl CalcUnit {
    fn from(tks: &[Token]) -> Option<(Self, usize)> {
        let mut offset = 0;
        // 0 -> Literal : Literal
        // 0 -> 1       : +/-
        // 0 -> 2       : Identifier
        // 0 -> Tuple   : Tuple
        // 1 -> NegVal  : Literal/Identifier
        // 2 -> Function: ~Tuple
        // 2 -> Identifier: Else
        let mut state = 0;
        let mut id = None;
        let mut is_neg = false;
        while offset < tks.len() && !tks[offset].is_eof() {
            if tks[offset].is_skipped() {
                if state != 2 {
                    offset += 1;
                    continue;
                } else {
                    return Some((CalcUnit::Identifier(id?), offset));
                }
            }
            match state {
                0 if tks[offset].is_literal() => {
                    let it = tks[offset].literal.clone()?;
                    offset += 1;
                    return Some((CalcUnit::Literal(it), offset));
                }
                0 if tks[offset].is_identifier() => {
                    if let Some(Literal::Identifier(i)) = &tks[offset].literal {
                        offset += 1;
                        id = Some(i.clone());
                        state = 2;
                        continue;
                    }
                }
                0 if tks[offset].is_pos_neg() => {
                    if tks[offset].token_type == TokenType::Minus {
                        is_neg = true;
                    }
                    offset += 1;
                    state = 1;
                    continue;
                }
                0 => {
                    if let Some((tp, len)) = Tuple::from(&tks[offset..]) {
                        offset += len;
                        return Some((CalcUnit::Tuple(tp), offset));
                    }
                }
                1 if tks[offset].is_literal() => {
                    let it = tks[offset].literal.clone()?;
                    offset += 1;
                    if is_neg {
                        return Some((CalcUnit::NegVal(it), offset));
                    } else {
                        return Some((CalcUnit::Literal(it), offset));
                    }
                }
                1 if tks[offset].is_identifier() => {
                    if let Some(Literal::Identifier(id)) = tks[offset].literal.clone() {
                        offset += 1;
                        if is_neg {
                            return Some((CalcUnit::NegVar(id), offset));
                        } else {
                            return Some((CalcUnit::Identifier(id), offset));
                        }
                    }
                }
                2 => {
                    if let Some((tp, len)) = Tuple::from(&tks[offset..]) {
                        offset += len;
                        return Some((CalcUnit::Function(id?, tp), offset));
                    }
                    return Some((CalcUnit::Identifier(id?), offset));
                }
                _ => {}
            }
            return None;
        }
        if state == 2 {
            return Some((CalcUnit::Identifier(id?), offset));
        }
        None
    }

    fn print(&self, level: usize) -> String {
        if level == 11 {
            match self {
                CalcUnit::Literal(l) => l.print(level),
                CalcUnit::Identifier(i) => {
                    format!("<span class='syntax_identifier'>{}</span>", i.name)
                }
                CalcUnit::NegVal(l) => {
                    format!("<span class='syntax_neg'>-{}</span>", l.print(level))
                }
                CalcUnit::NegVar(i) => format!(
                    "<span class='syntax_neg'>-<span class='syntax_identifier'>{}</span></span>",
                    i.name
                ),
                CalcUnit::Function(f, vars) => format!(
                    "<span class='syntax_fun'>{}{}</span>",
                    f.name,
                    vars.print(level)
                ),
                CalcUnit::Tuple(t) => t.print(level),
            }
        } else {
            match self {
                CalcUnit::Literal(l) => l.print(level),
                CalcUnit::Identifier(i) => format!("{}", i.name),
                CalcUnit::NegVal(l) => {
                    format!("-{}", l.print(level))
                }
                CalcUnit::NegVar(i) => format!("-{}", i.name),
                CalcUnit::Function(f, vars) => format!("{}{}", f.name, vars.print(level)),
                CalcUnit::Tuple(t) => t.print(level),
            }
        }
    }

    fn tree(&self, level: usize, html: bool) -> String {
        match self {
            CalcUnit::Literal(l) => " Literal ".to_string() + &tree_node(html, &l.print(3)),
            CalcUnit::Identifier(i) => " Identifier ".to_string() + &tree_node(html, &i.name),
            CalcUnit::NegVal(l) => " Literal Minus ".to_string() + &tree_node(html, &l.print(3)),
            CalcUnit::NegVar(i) => " Identifier Minus ".to_string() + &tree_node(html, &i.name),
            CalcUnit::Function(f, vars) => {
                let mut res = "+Function ".to_string() + &tree_node(html, &f.name) + "\n";
                res += &(INDENT.repeat(level) + "+---" + &vars.tree(level + 1, html));
                res
            }
            CalcUnit::Tuple(t) => t.tree(level, html),
        }
    }
}

impl Tuple {
    /// Warning: this func do not remove leading space before it, so it could return None
    fn from(tks: &[Token]) -> Option<(Self, usize)> {
        let mut offset = 0;
        // 0 -> 1 : (
        // 1 -> 2 : Expression
        // 3 -> 2 : Expression
        // 1 -> Ok: )
        // 2 -> Ok: )
        // 2 -> 3 : ,
        let mut state = 0;
        let mut exps = vec![];
        while offset < tks.len() && !tks[offset].is_eof() {
            if tks[offset].is_skipped() {
                offset += 1;
                continue;
            }
            match state {
                0 if tks[offset].token_type == TokenType::LeftParen => {
                    offset += 1;
                    state = 1;
                    continue;
                }
                1 | 2 if tks[offset].token_type == TokenType::RightParen => {
                    offset += 1;
                    return Some((Tuple { val: exps }, offset));
                }
                2 if tks[offset].token_type == TokenType::Comma => {
                    offset += 1;
                    state = 3;
                    continue;
                }
                1 | 3 => {
                    if let Some((exp, len)) = Expression::from(&tks[offset..]) {
                        offset += len;
                        state = 2;
                        exps.push(exp);
                        continue;
                    }
                }
                _ => {}
            }
            return None;
        }
        None
    }

    fn print(&self, level: usize) -> String {
        let mut res = "(".to_string();
        for i in 0..self.val.len() {
            res += &self.val[i].print(level);
            if i + 1 < self.val.len() {
                res += ", ";
            }
        }
        if level == 11 {
            format!("<span class='syntax_tuple'>{}</span>", res + ")")
        } else {
            res + ")"
        }
    }

    fn tree(&self, level: usize, html: bool) -> String {
        let mut res = "+Tuple ".to_string() + &tree_node(html, &self.val.len().to_string());
        for i in self.val.iter() {
            res += "\n";
            res += &(INDENT.repeat(level) + "+---" + &i.tree(level + 1, html));
        }
        res
    }
}

const INDENT: &str = "|   ";
fn tree_node(html: bool, name: &str) -> String {
    if html {
        format!("<span class='tree_syntax_node'>{name}</span>")
    } else {
        name.to_string()
    }
}
