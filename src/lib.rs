#[macro_use]
extern crate domain_derive;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate failure;

pub mod errors;
// Re-publish because of requirement by derive that Error be published to root.
pub use errors::Error;

pub mod domain;
pub mod application;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
