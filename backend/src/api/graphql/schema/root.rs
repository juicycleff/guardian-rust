use juniper::{EmptySubscription, RootNode};

use crate::api::graphql::schema::context::Context;
use crate::api::graphql::schema::mutation::MutationRoot;
use crate::api::graphql::schema::query::QueryRoot;

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot, EmptySubscription::new())
}
