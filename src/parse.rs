pub use crate::tokenize::Token;
pub use crate::evaluate::Thunk;

/// Type alias for a peekable stream of Tokens
type TokenStream<'a> = std::iter::Peekable<std::slice::Iter<'a, Token>>;

/// The starting point of the grammar, encompasses the entire input
#[derive(Debug, PartialEq)]
pub enum Start {
    /// Start = AddLayer ;
    Base(Thunk<AddLayer>),
}

/// Represents the layer that adding and subtracting are applied at
#[derive(Debug, PartialEq)]
pub enum AddLayer {
    /// AddLayer = MultLayer ;
    Base(Thunk<MultLayer>),
    /// AddLayer = AddLayer,  AddOperator, MultLayer ;
    Recurse(Thunk<Self>, AddOperator, Thunk<MultLayer>),

}

/// Represents the layer that multiplying and dividing are applied at
#[derive(Debug, PartialEq)]
pub enum MultLayer {
    /// MultLayer = Roll ;
    Base(Thunk<Roll>),
    /// MultLayer = [ MultLayer, MultOperator ], Roll ;
    Recurse(Thunk<Self>, MultOperator, Thunk<Roll>),
}

/// Represents rolling a die, and the parts of a roll
#[derive(Debug, PartialEq)]
pub enum Roll {
    /// Roll = SubExpression ;
    Base(Thunk<SubExpression>),
    /// Roll = Roll, "d", SubExpression ;
    Recurse(Thunk<Self>, RollOperator, Thunk<SubExpression>),
}

/// Represents a sub-expression
#[derive(Debug, PartialEq)]
pub enum SubExpression {
    /// SubExpression = Number ;
    Base(Thunk<Number>),
    /// SubExpression = "(", Start, ")" ;
    Recurse(Thunk<Start>),
}

/// Represents a number
#[derive(Debug, PartialEq)]
pub enum Number {
    /// Number = { "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "%" }+ ;
    StringRepresentation(String),
}

/// Represents an operator that has addition-level precedence
#[derive(Debug, PartialEq)]
pub enum AddOperator {
    /// AddOperator = "+" ;
    Add,
    /// AddOperator = "-" ;
    Subtract,
}

/// Represents an operator that has multiplication-level precedence
#[derive(Debug, PartialEq)]
pub enum MultOperator {
    /// MultOperator = "*";
    Multiply,
    /// MultOperator = "/" ;
    Divide,
}

/// This is included for symmetry with AddOperator and MultOperator
#[derive(Debug, PartialEq)]
pub enum RollOperator {
    /// RollOperator = "d" ;
    D,
}

/// Parse a tokenized input into an expression tree
pub fn parse(tokenized: Vec<Token>) -> Result<Thunk<Start>, String> {
    // An iterator over the tokens that were read
    let mut tokens = tokenized.iter().peekable();
    // Parse a Start (this is the root of the grammar)
    let start = Start::parse(&mut tokens);
    // If there are remaining tokens
    if let Some(token) = tokens.next() {
        // Report error
        Err(format!("Parse Error: Expected end of stream, found {:?}", token))
    }
    // Otherwise
    else {
        // Pass the parse-tree out
        start
    }
}

impl Start {
    fn parse(mut tokens: &mut TokenStream) -> Result<Thunk<Self>, String> {
        Ok(
            Thunk::new(
                Self::Base(
                    AddLayer::parse(&mut tokens)?
                )
            )
        )
    }
}

macro_rules! parse_left_assoc_infix {
    ($layer:ty, $base:ty, $operator:ty, $operator_token:pat) => {
        fn parse(mut tokens: &mut TokenStream) -> Result<Thunk<Self>, String> {
            let base = <$base>::parse(&mut tokens)?;
            match tokens.peek() {
                Some($operator_token) => {
                    let mut left = Thunk::new(
                        <$layer>::Base(base)
                    );
                    while let Some($operator_token) = tokens.peek() {
                        let op = <$operator>::parse(&mut tokens)?;
                        let right = <$base>::parse(&mut tokens)?;
                        left = Thunk::new(
                            <$layer>::Recurse(
                                left,
                                op,
                                right,
                            )
                        );
                    }
                    Ok(left)
                },
                _ => Ok(
                    Thunk::new(
                        <$layer>::Base(base),
                    )
                ),
            }
        }
    }
}

impl AddLayer {
    parse_left_assoc_infix!(AddLayer, MultLayer, AddOperator, Token::AddOperator(_));
}

impl MultLayer {
    parse_left_assoc_infix!(MultLayer, Roll, MultOperator, Token::MultOperator(_));
}

impl Roll {
    parse_left_assoc_infix!(Roll, SubExpression, RollOperator, Token::RollSeparator(_));
}

impl SubExpression {
    fn parse(mut tokens: &mut TokenStream) -> Result<Thunk<Self>, String> {
        match tokens.peek() {
            Some(Token::OpenParenthesis(_)) => {
                tokens.next();
                let nested = Start::parse(&mut tokens)?;
                match tokens.next() {
                    Some(Token::CloseParenthesis(_)) => Ok(
                        Thunk::new(
                            SubExpression::Recurse(nested)
                        )
                    ),
                    Some(token) => Err(format!("Parse Error: Expected \")\", found {:?}", token)),
                    None => Err("Parse Error: Expected \")\", found end of stream".to_string()),
                }
            },
            Some(Token::Number(_)) => {
                Ok(
                    Thunk::new(
                        SubExpression::Base(
                            Number::parse(&mut tokens)?
                        )
                    )
                )
            },
            Some(_) => Err(format!("Parse Error: Expected nested start or \"(\", found: {:?}", tokens.next())),
            None => Err("Parse Error: Expected nested start or \"(\", found end of stream".to_string()),
        }
    }
}

impl Number {
    fn parse(tokens: &mut TokenStream) -> Result<Thunk<Self>, String> {
        match tokens.next() {
            Some(Token::Number(number)) => Ok(
                Thunk::new(
                    Number::StringRepresentation(number.clone()),
                )
            ),
            Some(token) => Err(format!("Parse Error: Expected Number, found {:?}", token)),
            None => Err("Parse Error: Expected Number, found end of stream".to_string()),
        }
    }
}

impl AddOperator {
    fn parse(tokens: &mut TokenStream) -> Result<Self, String> {
        match tokens.next() {
            Some(Token::AddOperator(op)) => match op.as_ref() {
                "+" => Ok(AddOperator::Add),
                "-" => Ok(AddOperator::Subtract),
                _ => Err(format!("Parse Error: Expected \"+\" or \"-\", found {}", op)),
            }
            Some(token) => Err(format!("Parse Error: Expected AddOperator, found {:?}", token)),
            None => Err("Parse Error: Expected AddOperator, found end of stream".to_string()),
        }
    }
}

impl MultOperator {
    fn parse(tokens: &mut TokenStream) -> Result<Self, String> {
        match tokens.next() {
            Some(Token::MultOperator(op)) => match op.as_ref() {
                "*" => Ok(MultOperator::Multiply),
                "/" => Ok(MultOperator::Divide),
                _ => Err(format!("Parse Error: Expected \"*\" or \"/\", found {}", op)),
            }
            Some(token) => Err(format!("Parse Error: Expected MultOperator, found {:?}", token)),
            None => Err("Parse Error: Expected MultOperator, found end of stream".to_string()),
        }
    }
}

impl RollOperator {
    fn parse(tokens: &mut TokenStream) -> Result<Self, String> {
        match tokens.next() {
            Some(Token::RollSeparator(op)) => match op.as_ref() {
                "d" => Ok(RollOperator::D),
                _ => Err(format!("Parse Error: Expected \"d\", found {}", op)),
            }
            Some(token) => Err(format!("Parse Error: Expected RollOperator, found {:?}", token)),
            None => Err("Parse Error: Expected RollOperator, found end of stream".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parse::{parse, Token, Thunk};
    use crate::parse::{Start, AddLayer, MultLayer, Roll, SubExpression, Number, AddOperator, MultOperator, RollOperator};

    #[test]
    fn single_number() {
        assert_eq!(
            Ok(Thunk::new(Start::Base(
                Thunk::new(AddLayer::Base(
                    Thunk::new(MultLayer::Base(
                        Thunk::new(Roll::Base(
                            Thunk::new(SubExpression::Base(
                                Thunk::new(Number::StringRepresentation(
                                    "1".to_string()
                                ))
                            ))
                        ))
                    ))
                ))
            ))),
            parse(vec![Token::Number("1".to_string())])
        )
    }

    #[test]
    fn multi_roll() {
        assert_eq!(
	    Ok(Thunk::new(Start::Base(
		Thunk::new(AddLayer::Base(
		    Thunk::new(MultLayer::Base(
			Thunk::new(Roll::Recurse(
			    Thunk::new(Roll::Recurse(
				Thunk::new(Roll::Base(
				    Thunk::new(SubExpression::Base(
					Thunk::new(Number::StringRepresentation(
					    "1".to_string()
					))
				    ))
				)),
                                RollOperator::D,
				Thunk::new(SubExpression::Base(
				    Thunk::new(Number::StringRepresentation(
					"2".to_string()
				    ))
				))
			    )),
                            RollOperator::D,
			    Thunk::new(SubExpression::Base(
				Thunk::new(Number::StringRepresentation(
				    "3".to_string()
				))
			    ))
			))
		    ))
		))
	    ))),
            parse(
		vec![
		    Token::Number("1".to_string()),
		    Token::RollSeparator("d".to_string()),
		    Token::Number("2".to_string()),
		    Token::RollSeparator("d".to_string()),
		    Token::Number("3".to_string()),
		]
	    )
        )
    }

    #[test]
    fn multi_mult() {
        assert_eq!(
            Ok(Thunk::new(Start::Base(
                Thunk::new(AddLayer::Base(
                    Thunk::new(MultLayer::Recurse(
                        Thunk::new(MultLayer::Recurse(
                            Thunk::new(MultLayer::Base(
                                Thunk::new(Roll::Base(
                                    Thunk::new(SubExpression::Base(
                                        Thunk::new(Number::StringRepresentation(
                                            "1".to_string()
                                        ))
                                    ))
                                ))
                            )),
                            MultOperator::Multiply,
                            Thunk::new(Roll::Base(
                                Thunk::new(SubExpression::Base(
                                    Thunk::new(Number::StringRepresentation(
                                        "2".to_string()
                                    ))
                                ))
                            )),
                        )),
                        MultOperator::Divide,
                        Thunk::new(Roll::Base(
                            Thunk::new(SubExpression::Base(
                                Thunk::new(Number::StringRepresentation(
                                    "3".to_string()
                                ))
                            ))
                        )),
                    ))
                ))
            ))),
            parse(
		vec![
		    Token::Number("1".to_string()),
		    Token::MultOperator("*".to_string()),
		    Token::Number("2".to_string()),
		    Token::MultOperator("/".to_string()),
		    Token::Number("3".to_string()),
		]
	    )
        )
    }

    #[test]
    fn add_and_multiply() {
        assert_eq!(
            Ok(Thunk::new(Start::Base(
                Thunk::new(AddLayer::Recurse(
                    Thunk::new(AddLayer::Base(
                        Thunk::new(MultLayer::Recurse(
                            Thunk::new(MultLayer::Base(
                                Thunk::new(Roll::Base(
                                    Thunk::new(SubExpression::Base(
                                        Thunk::new(Number::StringRepresentation(
                                            "1".to_string()
                                        ))
                                    ))
                                ))
                            )),
                            MultOperator::Multiply,
                            Thunk::new(Roll::Base(
                                Thunk::new(SubExpression::Base(
                                    Thunk::new(Number::StringRepresentation(
                                        "2".to_string()
                                    ))
                                ))
                            )),
                        ))
                    )),
                    AddOperator::Add,
                    Thunk::new(MultLayer::Base(
                        Thunk::new(Roll::Base(
                            Thunk::new(SubExpression::Base(
                                Thunk::new(Number::StringRepresentation(
                                    "3".to_string()
                                ))
                            ))
                        ))
                    )),
                ))
            ))),
            parse(
		vec![
		    Token::Number("1".to_string()),
		    Token::MultOperator("*".to_string()),
		    Token::Number("2".to_string()),
		    Token::AddOperator("+".to_string()),
		    Token::Number("3".to_string()),
		]
	    )
        )
    }
}
