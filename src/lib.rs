#![deny(missing_docs)]
//! An encoder and decoder for FITS images.
//!
//! The *Flexible Image Transport System* ([FITS](https://en.wikipedia.org/wiki/FITS)) is
//! > an open standard defining a digital file format useful for storage,
//! > transmission and processing of scientific and other images.

#[macro_use]
extern crate nom;

pub mod parser;
pub mod types;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
