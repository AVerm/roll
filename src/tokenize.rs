/// Represents a single segment of the input string
#[derive(Debug, PartialEq)]
pub enum Token {
    /// The separator character used in a roll
    RollSeparator(String),
    /// The character used to open a layer of recursion
    OpenParenthesis(String),
    /// The character used to close a layer of recursion
    CloseParenthesis(String),
    /// A numerical constant
    Number(String),
    /// An addition or subtraction operator
    AddOperator(String),
    /// A multiplication or division operator
    MultOperator(String),
    /// An undefined token, brought along for later processing
    Undefined(String),
}

/// Take an input string and turn it into a list of tokens
pub fn tokenize(input: String) -> Vec<Token> {
    // The tokens that will be returned from the function
    let mut tokens = Vec::new();
    // A list of characters to turn into tokens
    let mut characters = input.chars().peekable();
    // This pattern is used so that the list can be advanced from inside the loop
    while let Some(ch) = characters.peek() {
        // Ignore whitespace
        if ch.is_whitespace() {
            (&mut characters).next();
            continue;
        }
        // Add a token to the list
        tokens.push(
            // What kind of character is it?
            match ch {
                'd' => Token::RollSeparator(characters.next().unwrap().to_string()),
                '(' => Token::OpenParenthesis(characters.next().unwrap().to_string()),
                ')' => Token::CloseParenthesis(characters.next().unwrap().to_string()),
                // If it is a numerical constant,
                '0'..='9' | '%' => Token::Number(
                    // Hand off to the parsing function
                    parse_number(&mut characters)
                ),
                '+' | '-' => Token::AddOperator(characters.next().unwrap().to_string()),
                '*' | '/' => Token::MultOperator(characters.next().unwrap().to_string()),
                // If it matched no pattern, bring it for processing
                _ => Token::Undefined(ch.to_string()),
            }
        );
    }
    tokens
}

/// Parses the next numerical constant into a string.
fn parse_number(characters: &mut std::iter::Peekable<std::str::Chars<'_>>) -> String {
    let mut accumulator = Vec::new();
    // Take the characters
    while let Some(c) = characters.peek() {
        match c {
            // That are also numerical constants
            '0'..='9' | '%' => accumulator.push(
                characters.next().unwrap()
            ),
            _ => break,
        }
    }
    accumulator.iter().collect()
}
