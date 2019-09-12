#[macro_use]
extern crate domain_derive;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate failure;

pub mod commands;
pub use commands::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
