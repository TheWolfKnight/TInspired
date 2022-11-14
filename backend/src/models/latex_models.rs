

use serde_json::value;

use super::errors::ModelErrors;
use std::fmt::{self, write};


pub trait LatexStructure
where
    Self: fmt::Display
{
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


#[derive(Debug)]
pub enum Operant {
    Plus,
    Minus,
    Mult,
    Div,
    Pow,
    Root,
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


#[derive(Debug)]
pub struct Minus<T, E>
where
    T: LatexStructure,
    E: LatexStructure
{
  l: Value<T>,
  r: Value<E>,
}


#[derive(Debug)]
pub struct Times<T, E>
where
    T: LatexStructure,
    E: LatexStructure
{
    l: Value<T>,
    r: Value<E>,
}


#[derive(Debug)]
pub struct Division<T, E>
where
    T: LatexStructure,
    E: LatexStructure
{
    l: Value<T>,
    r: Value<E>,
}


#[derive(Debug)]
pub struct Power<T, E>
where
    T: LatexStructure,
    E: LatexStructure
{
    l: Value<T>,
    r: Value<E>,
}


#[derive(Debug)]
pub struct Root<T, E>
where
    T: LatexStructure,
    E: LatexStructure
{
    l: Value<T>,
    r: Value<E>,
}


#[derive(Debug)]
pub struct Parentheses<T, E>
where
    T: LatexStructure,
    E: LatexStructure
{
    l: Value<T>,
    o: Operant,
    r: Value<E>
}

// STRUCT SECTION: END

// IMPL FOR EMPTY: BEGIN
impl LatexStructure for Empty {
  fn solve( &self ) -> Result<f64, ModelErrors> {
    if false {
      return Ok(1.);
    }
    return Err(ModelErrors::EmptyStructError());
  }
}


impl fmt::Display for Empty {
    fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
        write!(f, "Empty")
    }
}

// IMPL FOR EMPT: END

// IMPL FOR PLUS: BEGIN
impl<T, E> Plus<T, E>
where
  T: LatexStructure,
  E: LatexStructure
{
  pub fn new( l: Value<T>, r: Value<E> ) -> Self {
    Self {l, r}
  }

  pub fn change_l( &mut self, l: Value<T> ) {
    self.l = l;
  }

  pub fn change_r( &mut self, r: Value<E> ) {
    self.r = r;
  }

}


impl<T, E> LatexStructure for Plus<T, E>
where
  T: LatexStructure + fmt::Display + fmt::Debug,
  E: LatexStructure + fmt::Display + fmt::Debug,
{
  fn solve( &self ) -> Result<f64, ModelErrors> {
    let mut l_val: f64;
    let mut r_val: f64;
    match &self.l {
      Value::Pure(i) => l_val = *i,
      Value::Derived(v) => {
        match v.solve() {
          Ok(i) => l_val = i,
          Err(e) => return Err(e),
        }
      },
      Value::Null => return Err(ModelErrors::CalculationError("left branch of the Plus structure may not be null".to_string())),
    }

    match &self.r {
      Value::Pure(i) => r_val = *i,
      Value::Derived(v) => {
        match v.solve() {
            Ok(i) => r_val = i,
            Err(e) => return Err(e),
        }
      },
      Value::Null => r_val = 0.,
    }

    Ok(l_val + r_val)
  }
}

impl<T, E> fmt::Display for Plus<T, E>
where
  T: LatexStructure + fmt::Display + fmt::Debug,
  E: LatexStructure + fmt::Display + fmt::Debug,
{
  fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
      write!(f, "Plus(l={:?}, r={:?})", self.l, self.r)
  }
}

// IMPL FOR PLUS: END

// IMPL FOR MINUS: BEGIN

impl<T, E> Minus<T, E>
where
  T: LatexStructure,
  E: LatexStructure,
{
  pub fn new(l: Value<T>, r: Value<E>) -> Self {
    Self {l, r}
  }

  pub fn change_l( &mut self, l: Value<T> ) {
    self.l = l;
  }

  pub fn change_r( &mut self, r: Value<E> ) {
    self.r = r;
  }
}


impl<T, E> LatexStructure for Minus<T, E>
where
  T: LatexStructure + fmt::Display + fmt::Debug,
  E: LatexStructure + fmt::Display + fmt::Debug,
{
  fn solve( &self ) -> Result<f64, ModelErrors> {
    let mut l_val: f64;
    let mut r_val: f64;
    match &self.l {
      Value::Pure(i) => l_val = *i,
      Value::Derived(v) => {
        match v.solve() {
          Ok(i) => l_val = i,
          Err(e) => return Err(e),
        }
      },
      Value::Null => return Err(ModelErrors::CalculationError("left branch of the Plus structure may not be null".to_string())),
    }

    match &self.r {
      Value::Pure(i) => r_val = *i,
      Value::Derived(v) => {
        match v.solve() {
            Ok(i) => r_val = i,
            Err(e) => return Err(e),
        }
      },
      Value::Null => r_val = 0.,
    }

    Ok(l_val - r_val)
  }
}

impl<T, E> fmt::Display for Minus<T, E>
where
  T: LatexStructure + fmt::Display + fmt::Debug,
  E: LatexStructure + fmt::Display + fmt::Debug,
{
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "Minus(l={:?}, r={:?})", self.l, self.r)
  }
}

// IMPL FOR MINUS: END

// IMPL FOR TIMES: BEGIN

impl<T, E> Times<T, E>
where
  T: LatexStructure,
  E: LatexStructure,
{
  pub fn new( l: Value<T>, r: Value<E> ) -> Self {
    Self {l, r}
  }

  pub fn change_l( &mut self, l: Value<T> ) {
    self.l = l;
  }

  pub fn change_r( &mut self, r: Value<E>) {
    self.r = r;
  }

}

impl<T, E> LatexStructure for Times<T, E>
where
  T: LatexStructure + fmt::Display + fmt::Debug,
  E: LatexStructure + fmt::Display + fmt::Debug,
{
    fn solve( &self ) -> Result<f64, ModelErrors> {
        todo!();
    }
}

impl<T, E> fmt::Display for Times<T, E>
where
  T: LatexStructure + fmt::Display + fmt::Debug,
  E: LatexStructure + fmt::Display + fmt::Debug,
{
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "Times(l={:?}, r={:?})", self.l, self.r)
  }
}

// IMPL FOR TIMES: END
