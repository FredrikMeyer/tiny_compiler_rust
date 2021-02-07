fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
pub enum Keyword {
    IF,
    ENDIF,
    THEN,
    GOTO,
}

impl Keyword {
    pub fn from_string(name: &String) -> Option<Keyword> {
        match name.to_lowercase().as_str() {
            "if" => Some(Keyword::IF),
            "endif" => Some(Keyword::ENDIF),
            "then" => Some(Keyword::THEN),
            "goto" => Some(Keyword::GOTO),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum Token {
    EOF,
    SLASH,
    NEWLINE,
    GT,
    GTEQ,
    LT,
    LTEQ,
    NOTEQ,
    EQ,
    EQEQ,
    PLUS,
    MINUS,
    ASTERISK,
    KEYWORD(Keyword),
    IDENTIFIER(String),
    NUMBER(String),
}

pub struct Lexer {
    input: String,
    pub cur_char: Option<char>,
    cur_pos: usize,
}

impl Lexer {
    pub fn new(input: &String) -> Lexer {
        Lexer {
            input: input.clone(),
            cur_char: input.chars().nth(0),
            cur_pos: 0,
        }
    }
    pub fn next_char(&mut self) {
        self.cur_pos += 1;
        if self.cur_pos > self.input.len() {
            self.cur_char = None
        } else {
            self.cur_char = self.input.chars().nth(self.cur_pos);
        }
    }

    pub fn peek(&self) -> Option<char> {
        if self.cur_pos + 1 > self.input.len() {
            return None;
        } else {
            self.input.chars().nth(self.cur_pos + 1)
        }
    }

    pub fn abort(&self, message: String) {}

    pub fn skip_comment(&self) {}

    fn skip_whitespace(&mut self) {
        while self.cur_char == Some(' ')
            || self.cur_char == Some('\t')
            || self.cur_char == Some('\r')
        {
            self.next_char()
        }
    }

    pub fn get_token(&mut self) -> Option<Token> {
        // kan prøve å ha valid/valid token-klasse
        self.skip_whitespace();

        if let Some(c) = self.cur_char {
            match c {
                '+' => Some(Token::PLUS),
                '-' => Some(Token::MINUS),
                '*' => Some(Token::ASTERISK),
                '=' => match self.peek() {
                    Some('=') => {
                        self.next_char();
                        Some(Token::EQ)
                    }
                    _ => Some(Token::EQ),
                },
                '/' => Some(Token::SLASH),
                '\n' => Some(Token::NEWLINE),
                '>' => match self.peek() {
                    Some('=') => {
                        self.next_char();
                        Some(Token::GTEQ)
                    }
                    _ => Some(Token::GT),
                },
                '<' => match self.peek() {
                    Some('=') => {
                        self.next_char();
                        Some(Token::LTEQ)
                    }
                    _ => Some(Token::LT),
                },
                '!' => match self.peek() {
                    Some('=') => {
                        self.next_char();
                        Some(Token::NOTEQ)
                    }
                    _ => None,
                },
                x if x.is_digit(10) => {
                    let start_pos = self.cur_pos;
                    while self.peek().map_or(false, |v| v.is_digit(10)) {
                        self.next_char();
                    }
                    if self.peek() == Some('.') {
                        self.next_char();
                        if !self.peek().map_or(false, |v| v.is_digit(10)) {
                            return None;
                        }
                        while self.peek().map_or(false, |v| v.is_digit(10)) {
                            self.next_char();
                        }
                    }
                    let token_text = self.input.get(start_pos..self.cur_pos + 1);
                    return Some(Token::NUMBER(String::from(token_text.unwrap())));
                }
                x if x.is_alphabetic() => {
                    let start_pos = self.cur_pos;
                    while self.peek().map_or(false, |v| v.is_alphanumeric()) {
                        self.next_char()
                    }
                    let token_text = self.input.get(start_pos..self.cur_pos + 1);
                    let word = String::from(token_text.unwrap());

                    let token = Keyword::from_string(&word)
                        .map(|k| Token::KEYWORD(k))
                        .or(Some(Token::IDENTIFIER(word.clone())));

                    return token;
                }
                _ => None,
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_res() {
        let test_input = "\
LET foobar = 123
";
        let mut lexer: Lexer = Lexer::new(&String::from(test_input));

        println!("start");
        while lexer.peek().is_some() {
            print!("{}", lexer.cur_char.unwrap_or('0'));
            lexer.next_char();
        }
        println!("\nend");
    }

    #[test]
    fn test_read_token() {
        let input = "+- */ >>= = != 105+123.42\n123 456\n if ifn ";
        let mut lexer = Lexer::new(&String::from(input));

        let mut token = lexer.get_token();
        while lexer.peek().is_some() {
            token = lexer.get_token();
            println!("{:?}", token);
            lexer.next_char();
        }
        assert!(false)
    }
}
