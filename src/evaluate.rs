use num_bigint::BigInt;
use crate::parse::{Start, AddLayer, MultLayer, Roll, SubExpression, Number, AddOperator, MultOperator, RollOperator};

pub fn evaluate(tree: Thunk<Start>) -> Result<BigInt, String> {
    tree.consume()
}

/// A trait that represents something that can be evaluated
/// into a BigInt at some point in time
pub trait Delayable {
    fn evaluate(self) -> Result<BigInt, String>;
}

/// Some computation that is stored and evaluated at a
/// later point
#[derive(Debug, PartialEq)]
pub struct Thunk<T: Delayable> {
    /// The thing that can be evaluated at a later point
    delayed: Box<T>,
    /// The result of evaluating the stored expression
    result: Option<Result<BigInt, String>>,
}

impl<T: Delayable + Sized> Thunk<T> {
    /// Create a new Thunk from an item of a known size
    pub fn new(delayed: T) -> Self {
        Self {
            /// Store the computation for later
            delayed: Box::new(delayed),
            /// It is not yet evaluated
            result: None,
        }
    }
    /// Consume the Thunk and return the result
    pub fn consume(self) -> Result<BigInt, String> {
        (*self.delayed).evaluate()
    }
}

impl Delayable for Start {
    fn evaluate(self) -> Result<BigInt, String> {
        unimplemented!()
    }
}

impl Delayable for AddLayer {
    fn evaluate(self) -> Result<BigInt, String> {
        unimplemented!()
    }
}

impl Delayable for MultLayer {
    fn evaluate(self) -> Result<BigInt, String> {
        unimplemented!()
    }
}

impl Delayable for Roll {
    fn evaluate(self) -> Result<BigInt, String> {
        unimplemented!()
    }
}

impl Delayable for SubExpression {
    fn evaluate(self) -> Result<BigInt, String> {
        unimplemented!()
    }
}

impl Delayable for Number {
    fn evaluate(self) -> Result<BigInt, String> {
        unimplemented!()
    }
}
