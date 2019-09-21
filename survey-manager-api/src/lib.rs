#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate derive_more;

pub mod inputs;
pub mod commands;
pub mod queries;
pub mod generate;
pub mod extractors;
pub mod responders;
pub mod error;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
