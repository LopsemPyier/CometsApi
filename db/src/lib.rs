mod db;
pub mod user;
pub mod model;

pub use db::Database;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
