pub use crate::tokenize::Token;
pub use crate::evaluate::Thunk;

/// Type alias for a peekable stream of Tokens
type TokenStream<'a> = std::iter::Peekable<std::slice::Iter<'a, Token>>;

/// The starting point of the grammar, encompasses the entire input
#[derive(Debug)]
pub enum Start {
    /// Start = AddLayer ;
    Base(Thunk<AddLayer>),
}

/// Represents the layer that adding and subtracting are applied at
#[derive(Debug)]
pub enum AddLayer {
    /// AddLayer = MultLayer ;
    Base(Thunk<MultLayer>),
    /// AddLayer = AddLayer,  AddOperator, MultLayer ;
    Recurse(Thunk<Self>, AddOperator, Thunk<MultLayer>),

}

/// Represents the layer that multiplying and dividing are applied at
#[derive(Debug)]
pub enum MultLayer {
    /// MultLayer = Roll ;
    Base(Thunk<Roll>),
    /// MultLayer = [ MultLayer, MultOperator ], Roll ;
    Recurse(Thunk<Self>, MultOperator, Thunk<Roll>),
}

/// Represents rolling a die, and the parts of a roll
#[derive(Debug)]
pub enum Roll {
    /// Roll = SubExpression ;
    Base(Thunk<SubExpression>),
    /// Roll = Roll, "d", SubExpression ;
    Recurse(Thunk<Self>, RollOperator, Thunk<SubExpression>),
}

/// Represents a sub-expression
#[derive(Debug)]
pub enum SubExpression {
    /// SubExpression = Number ;
    Base(Thunk<Number>),
    /// SubExpression = "(", Start, ")" ;
    Recurse(Thunk<Start>),
}

/// Represents a number
#[derive(Debug)]
pub enum Number {
    /// Number = { "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "%" }+ ;
    StringRepresentation(String),
}

/// Represents an operator that has addition-level precedence
#[derive(Debug)]
pub enum AddOperator {
    /// AddOperator = "+" ;
    Add,
    /// AddOperator = "-" ;
    Subtract,
}

/// Represents an operator that has multiplication-level precedence
#[derive(Debug)]
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
    Start::parse(&mut tokens)
}

impl Start {
    fn parse(mut tokens: &mut TokenStream) -> Result<Thunk<Self>, String> {
        unimplemented!()
    }
}

impl AddLayer {
    fn parse(mut tokens: &mut TokenStream) -> Result<Thunk<Self>, String> {
        unimplemented!()
    }
}

impl MultLayer {
    fn parse(mut tokens: &mut TokenStream) -> Result<Thunk<Self>, String> {
        unimplemented!()
    }
}

impl Roll {
    fn parse(mut tokens: &mut TokenStream) -> Result<Thunk<Self>, String> {
        unimplemented!()
    }
}

impl SubExpression {
    fn parse(mut tokens: &mut TokenStream) -> Result<Thunk<Self>, String> {
        unimplemented!()
    }
}

impl Number {
    fn parse(tokens: &mut TokenStream) -> Result<Thunk<Self>, String> {
        unimplemented!()
    }
}
