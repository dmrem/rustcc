use crate::tokens;
use lazy_static::lazy_static;
use regex::Regex;

pub fn lex(input_str: &str) -> Vec<tokens::Token> {
    let mut curr_list = vec![];
    let mut s = input_str;

    lazy_static! {
        static ref REGEXES:Vec<Regex> = vec![
            Regex::new(r"^\s+").unwrap(), // whitespace
            Regex::new(r"^int ").unwrap(),
            Regex::new(r"^return ").unwrap(),
            Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*").unwrap(),
            Regex::new(r"^[0-9]+(?:\.[0-9]+)?").unwrap(),
            Regex::new(r"^\(").unwrap(),
            Regex::new(r"^\)").unwrap(),
            Regex::new(r"^\{").unwrap(),
            Regex::new(r"^\}").unwrap(),
            Regex::new(r"^\;").unwrap(),
        ];
    }

    loop {
        if s.len() == 0 {
            curr_list.push(tokens::Token::EOF);
            return curr_list;
        }

        let (matched_regex, capture) = {
            let x = REGEXES.iter().enumerate().find(|(_, reg)| reg.is_match(&s));
            match x {
                Some((pos, regex)) => (pos, regex.captures(s).unwrap()),
                None => panic!("No regex matched!")
            }
        };

        let token: tokens::Token = match matched_regex {
            0 => {
                let matched_len = capture[0].len();
                s = &s[matched_len..];
                continue;
            }
            1 => tokens::Token::KeywordInt,
            2 => tokens::Token::KeywordReturn,
            3 => tokens::Token::Identifier(capture[0].to_owned()),
            4 => tokens::Token::NumericConstant(capture[0].to_owned()),
            5 => tokens::Token::OpenParenthesis,
            6 => tokens::Token::CloseParenthesis,
            7 => tokens::Token::OpenCurlyBrace,
            8 => tokens::Token::CloseCurlyBrace,
            9 => tokens::Token::Semicolon,
            num => panic!("Unknown regex matched ({})!", num),
        };

        let matched_len = capture[0].len();
        curr_list.push(token);
        s = &s[matched_len..];
        continue;
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer;
    use crate::tokens::Token;

    #[test]
    fn test_if_it_runs_without_crashing() {
        let s = "int main() {\n    return 2;\n}";
        lexer::lex(s);
    }

    #[test]
    fn test_lex_one_token_keyword_int() {
        let s = "int ";
        let expected = vec![Token::KeywordInt, Token::EOF];
        let actual = lexer::lex(s);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_lex_one_token_keyword_return() {
        let s = "return ";
        let expected = vec![Token::KeywordReturn, Token::EOF];
        let actual = lexer::lex(s);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_lex_one_token_identifier_only_letters() {
        let s = "hello";
        let expected = vec![Token::Identifier("hello".to_string()), Token::EOF];
        let actual = lexer::lex(s);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_lex_one_token_identifier_letters_and_numbers() {
        let s = "hello1234";
        let expected = vec![Token::Identifier("hello1234".to_string()), Token::EOF];
        let actual = lexer::lex(s);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_lex_one_token_numeric_constant_no_decimal() {
        let s = "1234";
        let expected = vec![Token::NumericConstant("1234".to_string()), Token::EOF];
        let actual = lexer::lex(s);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_lex_one_token_numeric_constant_with_decimal() {
        let s = "1234.5678";
        let expected = vec![Token::NumericConstant("1234.5678".to_string()), Token::EOF];
        let actual = lexer::lex(s);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_lex_punctuation_tokens() {
        let s = "(){};";
        let expected = vec![
            Token::OpenParenthesis,
            Token::CloseParenthesis,
            Token::OpenCurlyBrace,
            Token::CloseCurlyBrace,
            Token::Semicolon,
            Token::EOF
        ];
        let actual = lexer::lex(s);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_works_with_valid_simple_program() {
        let s = "int main() {\n    return 2;\n}";
        let expected: Vec<Token> = vec![
            Token::KeywordInt,
            Token::Identifier("main".to_string()),
            Token::OpenParenthesis,
            Token::CloseParenthesis,
            Token::OpenCurlyBrace,
            Token::KeywordReturn,
            Token::NumericConstant("2".to_string()),
            Token::Semicolon,
            Token::CloseCurlyBrace,
            Token::EOF
        ];
        let actual = lexer::lex(s);

        assert_eq!(expected, actual);
    }
}
