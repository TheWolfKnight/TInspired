
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ModelErrors {
  #[error("{0}")]
  CalculationError(String),

  #[error("You cannot solve an Empty struct")]
  EmptyStructError(),

  #[error("You cannot divide by zero")]
  DivisionByZeroError(),
}
