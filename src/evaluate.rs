use num_bigint::{BigInt, RandBigInt};
use num_traits::{Zero, One};
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
        match self {
            Self::Base(base_thunk) => base_thunk.consume(),
        }
    }
}

impl Delayable for AddLayer {
    fn evaluate(self) -> Result<BigInt, String> {
        match self {
            Self::Base(base_thunk) => base_thunk.consume(),
            Self::Recurse(left_thunk, op, right_thunk) => {
                let left = left_thunk.consume()?;
                let right = right_thunk.consume()?;
                match op {
                    AddOperator::Add => Ok(left + right),
                    AddOperator::Subtract => Ok(left - right),
                }
            },
        }
    }
}

impl Delayable for MultLayer {
    fn evaluate(self) -> Result<BigInt, String> {
        match self {
            Self::Base(base_thunk) => base_thunk.consume(),
            Self::Recurse(left_thunk, op, right_thunk) => {
                let left = left_thunk.consume()?;
                let right = right_thunk.consume()?;
                match op {
                    MultOperator::Multiply => Ok(left * right),
                    MultOperator::Divide => Ok(left / right),
                }
            },
        }
    }
}

impl Delayable for Roll {
    fn evaluate(self) -> Result<BigInt, String> {
        match self {
            Self::Base(base_thunk) => base_thunk.consume(),
            Self::Recurse(left_thunk, op, right_thunk) => {
                let left = left_thunk.consume()?;
                let right = right_thunk.consume()?;
                match op {
                    RollOperator::D => {
                        if left < BigInt::zero() {
                            Err(format!("Evaluation Error: Left side of a roll was less than 0, found {}", left))
                        } else {
                            let mut rng = rand::thread_rng();
                            let mut sum = BigInt::zero();
                            let mut dice_left = left;
                            let upper_bound_exclusive = right + 1;
                            while dice_left > BigInt::zero() {
                                sum += rng.gen_bigint_range(&BigInt::one(), &upper_bound_exclusive);
                                dice_left -= 1;
                            }
                            Ok(sum)
                        }
                    }
                }
            },
        }
    }
}

impl Delayable for SubExpression {
    fn evaluate(self) -> Result<BigInt, String> {
        match self {
            Self::Base(number_thunk) => number_thunk.consume(),
            Self::Recurse(start_thunk) => start_thunk.consume(),
        }
    }
}

impl Delayable for Number {
    fn evaluate(self) -> Result<BigInt, String> {
        let Self::StringRepresentation(string_rep) = self;
        if string_rep.is_empty() {
            Err("Evaluation Error: Found empty number".to_string())
        } else {
            let mut digits = string_rep.chars().peekable();
            let mut value = if digits.peek() == Some(&'%') {
                BigInt::one()
            }
            else {
                BigInt::zero()
            };
            while let Some(digit) = digits.next() {
                match digit {
                    '0' => value = (value * 10) + 0,
                    '1' => value = (value * 10) + 1,
                    '2' => value = (value * 10) + 2,
                    '3' => value = (value * 10) + 3,
                    '4' => value = (value * 10) + 4,
                    '5' => value = (value * 10) + 5,
                    '6' => value = (value * 10) + 6,
                    '7' => value = (value * 10) + 7,
                    '8' => value = (value * 10) + 8,
                    '9' => value = (value * 10) + 9,
                    '%' => value = value * 100,
                    _ => return Err(format!("Evaluation Error: Expected '0'-'9' or '%', found {}", digit)),
                }
            }
            Ok(value)
        }
    }
}
