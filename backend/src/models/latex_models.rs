

use super::errors::ModelErrors;
use std::fmt;


pub trait LatexStructure {
  fn solve( &self ) -> Result<f64, ModelErrors>;
}

#[derive(Debug)]
pub enum Value<T>
where
  T: LatexStructure
{
  Pure(f64),
  Derived(T),
  Null,
}


// STRUCT SECTION: BEGIN
#[derive(Debug)]
pub struct Empty;

#[derive(Debug)]
pub struct Plus<T, E>
where
  T: LatexStructure,
  E: LatexStructure
{
  l: Value<T>,
  r: Value<E>,
}

// STRUCT SECTION: END

impl LatexStructure for Empty {
  fn solve( &self ) -> Result<f64, ModelErrors> {
    if false {
      return Ok(1.);
    }
    return Err(ModelErrors::EmptyStructError());
  }
}

// IMPL FOR PLUS: BEGIN
impl<T, E> Plus<T, E>
where
  T: LatexStructure,
  E: LatexStructure
{
  pub fn new( l: Value<T>, r: Value<E> ) -> Self {
    Self {l, r}
  }

  pub fn change_l( &mut self, l: Value<T> ) -> () {
    self.l = l;
  }

  pub fn change_r( &mut self, r: Value<E> ) -> () {
    self.r = r;
  }

}


impl<T, E> LatexStructure for Plus<T, E>
where
  T: LatexStructure,
  E: LatexStructure
{
  fn solve( &self ) -> Result<f64, ModelErrors> {
    Ok(1.)
  }
}

impl<T, E> fmt::Display for Plus<T, E>
where
  T: LatexStructure + fmt::Display,
  E: LatexStructure + fmt::Display,
{
  fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
  }
}

// IMPL FOR PLUS: END
