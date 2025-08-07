use std::iter::Peekable;
use std::str::Chars;

#[derive(PartialEq)]
pub enum Token {
    Number(f64),
    StringLiteral(String),
    Word(String),
    LeftBrace,
    RightBrace,
    Colon,
    Semicolon,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    Comma,
    Ampersand
}

fn parse_number(first: char, chars: &mut Peekable<Chars<'_>>) -> Token {
    let mut num_str = first.to_string(); // start with the first digit
    let mut seen_dot = false;

    while let Some(&ch) = chars.peek() {
        match ch {
            '0'..='9' => {
                num_str.push(ch);
                chars.next();
            }
            '.' if !seen_dot => {
                seen_dot = true;
                num_str.push(ch);
                chars.next();
            }
            _ => break
        }
    }

    let parsed = num_str.parse::<f64>().unwrap_or_else(|_| {
        panic!("idk what a '{}' is. ain't no number for sure", num_str)
    });

    Token::Number(parsed)
}

fn parse_string(quote: char, chars: &mut Peekable<Chars<'_>>) -> Token {
    let mut content = String::new();
    let mut slash = false;

    while let Some(&ch) = chars.peek() {
        chars.next();

        if slash {
            content.push(ch) // THIS IS TEMPORARY!!!!!!!!!!!!!!!!!
        } else if ch == '\\' {
            slash = true;
        } else if ch == quote {
            break;
        } else {
            content.push(ch);
        }
    }
    Token::StringLiteral(content)
}

fn parse_word(firstchar: char, chars: &mut Peekable<Chars<'_>>) -> Token {
    let mut word = firstchar.to_string();
    
    while let Some(&ch) = chars.peek() {
        if ch.is_alphanumeric() || ch == '_' || ch == '-' { word.push(ch) }
        else { break };

        chars.next();
    }

    Token::Word(word)
}

pub fn tokenize(source: String) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = source.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            ' ' | '\n' | '\t' => continue,
            '"' | '\'' => tokens.push(parse_string(ch, &mut chars)),
            '0'..='9' => tokens.push(parse_number(ch, &mut chars)),
            'a'..='z' | 'A'..='Z' => tokens.push(parse_word(ch, &mut chars)),
            '(' => tokens.push(Token::LeftParen),
            ')' => tokens.push(Token::RightParen),
            ';' => tokens.push(Token::Semicolon),
            ':' => tokens.push(Token::Colon),
            '{' => tokens.push(Token::LeftBrace),
            '}' => tokens.push(Token::RightBrace),
            ',' => tokens.push(Token::Comma),
            _ => continue
        }
    }
    tokens
}


