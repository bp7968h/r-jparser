#[derive(PartialEq, Debug)]
enum TokenType{
    LeftCurlyBracket,
    RightCurlyBracket,
}

#[derive(Debug)]
struct Position(usize, usize);

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    position: Position,
}

pub fn tokenizer(contents: &str) -> Result<Vec<Token>, String> {
    let mut result_tokens: Vec<Token> = Vec::new();

    for (index, item) in contents.chars().enumerate(){
        let token_type: Option<TokenType> = match item {
            '{' => Some(TokenType::LeftCurlyBracket),
            '}' => Some(TokenType::RightCurlyBracket),
            item if item.is_whitespace() => None,
            _ => return Err(format!("Unexpected character '{}' at position {}", item, index)),

        };

        if let Some(token) = token_type {
            result_tokens.push(Token{token_type: token, position: Position(index, index)});
        }
    }

    Ok(result_tokens)
}