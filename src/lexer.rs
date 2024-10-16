use crate::tokens::Token;

pub struct Lexer {
    input: String,
    pos: usize,
    curr_char: Option<char>,
}

// mod tokens;

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lexer = Lexer {
            input,
            pos: 0,
            curr_char: None,
        };
        lexer.read_char();
        lexer
    }

    pub fn read_char(&mut self) {
        if self.pos >= self.input.len() {
            self.curr_char = None;
        } else {
            self.curr_char = Some(self.input.chars().nth(self.pos).unwrap());
        }
        self.pos += 1;
    }

    pub fn next_token(&mut self) -> Option<Token> {
        while let Some(c) = self.curr_char {
            match c {
                ' ' | '\n' | '\t' | '\r' => self.read_char(),
                '{' => {
                    self.read_char();
                    return Some(Token::LeftBrace);
                }
                '}' => {
                    self.read_char();
                    return Some(Token::RightBrace);
                }
                '[' => {
                    self.read_char();
                    return Some(Token::LeftBracket);
                }
                ']' => {
                    self.read_char();
                    return Some(Token::RightBracket);
                }
                ',' => {
                    self.read_char();
                    return Some(Token::COMMA);
                }
                ':' => {
                    self.read_char();
                    return Some(Token::COLON);
                }
                '"' => return Some(self.read_string()),
                '0'..='9' | '-' | '+' => return Some(self.read_number()),
                _ => {
                    println!("Invalid token!");
                    return None;
                }
            }
        }
        None
    }

    pub fn read_string(&mut self) -> Token {
        self.read_char();
        let mut res = String::new();
        while let Some(c) = self.curr_char {
            if c == '"' {
                break;
            }
            res.push(c);
            self.read_char();
        }
        self.read_char();
        Token::STRING(res)
    }

    pub fn read_number(&mut self) -> Token {
        let mut res: String = String::new();
        while let Some(c) = self.curr_char {
            if c.is_ascii_digit() || c == 'E' || c == 'e' || c == '+' || c == '-' || c == '.' {
                res.push(c);
                self.read_char();
            } else {
                break;
            }
        }
        Token::NUMBER(res.parse::<f64>().unwrap())
    }
}
