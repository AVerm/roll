/// Represents a single segment of the input string
#[derive(Debug)]
pub enum Token {
    /// An undefined token, brought along for later processing
    Undefined(String),
}

/// Take an input string and turn it into a list of tokens
pub fn tokenize(input: String) -> Vec<Token> {
    // The tokens that will be returned from the function
    let mut tokens = Vec::new();
    // A list of characters to turn into tokens
    let mut characters = input.chars();
    // This pattern is used so that the list can be advanced from inside the loop
    while let Some(ch) = characters.next() {
        // Add a token to the list
        tokens.push(
            // What kind of character is it?
            match ch {
                // If it matched no pattern, bring it for processing
                _ => Token::Undefined(ch.to_string()),
            }
        );
    }
    tokens
}
