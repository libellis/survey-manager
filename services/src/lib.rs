#[macro_use]
extern crate domain_derive;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate failure;

pub use survey_manager_domain::errors::*;

pub mod command_handler;
pub mod token;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
