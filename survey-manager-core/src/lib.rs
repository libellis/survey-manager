#[macro_use]
extern crate domain_derive;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate snafu;

pub mod errors;
// Re-publish because of requirement by derive that Error be published to root.
pub use errors::Error;

pub mod app_services;
pub mod survey;
pub mod dtos;
pub mod value_objects;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
