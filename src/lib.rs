#![feature(test)]
extern crate num;
#[macro_use]
extern crate lazy_static;

#[cfg(feature = "postgres")]
#[macro_use]
extern crate postgres as pg_crate;

extern crate serde;
#[macro_use]
extern crate serde_derive;

mod decimal;
mod error;

#[cfg(feature = "postgres")]
mod postgres;
#[cfg(feature = "serde")]
mod serde_types;

pub use decimal::Decimal;
pub use error::Error;
