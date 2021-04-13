mod query;
mod context;
mod mutation;

pub use query::Query as QueryRoot;
pub use mutation::Mutation as MutationRoot;
pub use context::ContextData;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
