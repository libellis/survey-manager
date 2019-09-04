#[macro_use]
extern crate domain_derive;

pub mod domain;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
