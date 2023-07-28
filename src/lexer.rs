#[derive(Debug)]
pub enum Token {
    Unknown(char),
    Space,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Number(i32)
}

pub struct Lexer {
    characters: Vec<char>,
    position: usize
}

impl Lexer {
    pub fn new(characters: Vec<char>) -> Lexer {
        Lexer {
            characters,
            position: 0,
        }
    }

    pub fn read_token(&mut self) -> Option<Token> {
        let Some(&character) = self.peek() else {
            return None;
        };
        if character.is_ascii_digit() {
            return Some(Token::Number(self.read_number()));
        }
        let token = match character {
            ' ' => Token::Space,
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Asterisk,
            '/' => Token::Slash,
            _   => Token::Unknown(character)
        };
        self.advance();
        Some(token)
    }

    fn read_number(&mut self) -> i32 {
        let mut number = 0;
        while let Some(&character) = self.peek() {
            match character.to_digit(10) {
                Some(digit) => {
                    number = number * 10 + digit as i32;
                },
                None => break
            }
            self.advance();
        }
        number
    }

    fn peek(&self) -> Option<&char> {
        self.characters.get(self.position)
    }

    fn advance(&mut self) {
        self.position += 1;
    }
}