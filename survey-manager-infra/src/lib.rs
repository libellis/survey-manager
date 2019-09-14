pub mod mysql_repos;
pub mod errors;
pub use errors::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
