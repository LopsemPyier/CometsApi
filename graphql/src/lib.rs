mod query;
mod mutation;

pub use query::Query as QueryRoot;
pub use mutation::Mutation as MutationRoot;
pub use schema::context::ContextData;