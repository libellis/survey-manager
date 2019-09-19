#[macro_use]
extern crate domain_derive;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate derive_more;

#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

pub mod inputs;
pub mod commands;
pub mod queries;
pub mod generate;
pub mod extractors;
pub mod error;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
