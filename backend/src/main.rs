#![allow(non_snake_case)]
#![allow(unused)]

pub mod models;
pub mod misc;

use models::latex_models::*;


fn main() {

  let mut h: Vec<&dyn LatexStructure> = Vec::new();
  let p: Plus<Empty, Empty> = Plus::new(Value::Pure(1.), Value::Pure(1.));

  h.push(&p);
  println!("{}", h[0]);

  let e: Empty = Empty;
  h[0] = &e;
  println!("{}", h[0]);
}
