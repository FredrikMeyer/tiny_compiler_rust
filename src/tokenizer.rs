#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq)]
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
    STRING(String),
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

    fn abort(&self, _message: String) {}

    fn skip_comment(&mut self) {
        if self.cur_char == Some('#') {
            while self.cur_char != Some('\n') {
                self.next_char()
            }
        }
    }

    fn skip_whitespace(&mut self) {
        while self.cur_char == Some(' ')
            || self.cur_char == Some('\t')
            || self.cur_char == Some('\r')
        {
            self.next_char()
        }
    }

    pub fn get_token(&mut self) -> Result<Token, &'static str> {
        self.skip_whitespace();
        self.skip_comment();

        let token: Result<Token, &'static str> = match self.cur_char {
            Some(c) => match c {
                '+' => Ok(Token::PLUS),
                '-' => Ok(Token::MINUS),
                '*' => Ok(Token::ASTERISK),
                '=' => match self.peek() {
                    Some('=') => {
                        self.next_char();
                        Ok(Token::EQEQ)
                    }
                    _ => Ok(Token::EQ),
                },
                '/' => Ok(Token::SLASH),
                '\n' => Ok(Token::NEWLINE),
                '>' => match self.peek() {
                    Some('=') => {
                        self.next_char();
                        Ok(Token::GTEQ)
                    }
                    _ => Ok(Token::GT),
                },
                '<' => match self.peek() {
                    Some('=') => {
                        self.next_char();
                        Ok(Token::LTEQ)
                    }
                    _ => Ok(Token::LT),
                },
                '!' => match self.peek() {
                    Some('=') => {
                        self.next_char();
                        Ok(Token::NOTEQ)
                    }
                    _ => Err("Unknown token !?"),
                },
                x if x.is_digit(10) => {
                    let start_pos = self.cur_pos;
                    while self.peek().map_or(false, |v| v.is_digit(10)) {
                        self.next_char();
                    }
                    if self.peek() == Some('.') {
                        self.next_char();
                        if !self.peek().map_or(false, |v| v.is_digit(10)) {
                            return Err("Unknown token 5.x");
                        }
                        while self.peek().map_or(false, |v| v.is_digit(10)) {
                            self.next_char();
                        }
                    }
                    let token_text = self.input.get(start_pos..self.cur_pos + 1);
                    return Ok(Token::NUMBER(String::from(token_text.unwrap())));
                }
                x if x.is_alphabetic() => {
                    let start_pos = self.cur_pos;
                    while self.peek().map_or(false, |v| v.is_alphanumeric()) {
                        self.next_char()
                    }
                    let token_text = self.input.get(start_pos..self.cur_pos + 1);
                    let word = String::from(token_text.unwrap());

                    let token = Keyword::from_string(&word)
                        .map_or(Token::IDENTIFIER(word.clone()), |k| Token::KEYWORD(k));

                    return Ok(token);
                }
                '\"' => {
                    let start_pos = self.cur_pos;
                    while self.peek().map_or(false, |v| v != '\"') {
                        self.next_char()
                    }
                    let token_text = self.input.get(start_pos + 1..self.cur_pos + 1);
                    return Ok(Token::STRING(String::from(token_text.unwrap())));
                }
                _ => Err("Unknown "),
            },
            None => Ok(Token::EOF),
        };
        // self.next_char(); // TODO fix this
        token
    }
}

impl Iterator for Lexer {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let curr = self.cur_char;
        self.next_char();
        curr
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
        let lexer: Lexer = Lexer::new(&String::from(test_input));

        let mut result = String::from("");
        for c in lexer {
            result.push(c);
        }

        assert_eq!(test_input, result);
    }

    #[test]
    fn test_read_token() {
        let input = "+- */ > >= = != 105+123.42\n123 456\n if ifn ";
        let mut lexer = Lexer::new(&String::from(input));

        let mut res_tokens = Vec::new();
        while lexer.peek().is_some() {
            if let Ok(t) = lexer.get_token() {
                res_tokens.push(t)
            }
            lexer.next_char();
        }

        let tokens: Vec<Token> = vec![
            Token::PLUS,
            Token::MINUS,
            Token::ASTERISK,
            Token::SLASH,
            Token::GT,
            Token::GTEQ,
            Token::EQ,
            Token::NOTEQ,
            Token::NUMBER("105".to_string()),
            Token::PLUS,
            Token::NUMBER("123.42".to_string()),
            Token::NEWLINE,
            Token::NUMBER("123".to_string()),
            Token::NUMBER("456".to_string()),
            Token::NEWLINE,
            Token::KEYWORD(Keyword::IF),
            Token::IDENTIFIER("ifn".to_string()),
        ];

        assert_eq!(tokens, res_tokens);
    }

    #[test]
    fn test_comment() {
        let input = "\
+
# something here
-
";
        let mut lexer = Lexer::new(&String::from(input));
        let mut res = Vec::new();
        while lexer.peek().is_some() {
            if let Ok(t) = lexer.get_token() {
                res.push(t);
            }
            lexer.next_char();
        }

        assert_eq!(
            res,
            vec![Token::PLUS, Token::NEWLINE, Token::NEWLINE, Token::MINUS]
        )
    }

    #[test]
    fn test_if_then() {
        let input = "IF+-123 foo*THEN/";
        let mut lexer = Lexer::new(&String::from(input));

        let mut token = lexer.get_token();
        while lexer.peek().is_some() {
            println!("{:?}", token);
            token = lexer.get_token();
            lexer.next_char();
        }
    }

    #[test]
    fn test_read_string() {
        let input = r#""some string""#;
        print!("{:?}", input);
        let mut lexer = Lexer::new(&String::from(input));

        let mut res = Vec::new();
        while lexer.peek().is_some() {
            if let Ok(t) = lexer.get_token() {
                res.push(t);
            }
            lexer.next_char();
        }
        print!("{:?}", res);
        assert_eq!(res, vec![Token::STRING("some string".to_string())]);
    }
}
