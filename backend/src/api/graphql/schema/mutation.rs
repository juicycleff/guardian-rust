use crate::api::graphql::schema::account_mutation::AccountMutations;
use crate::api::graphql::schema::context::Context;
use crate::api::graphql::schema::session_mutation::SessionMutations;
use juniper::FieldResult;

pub struct MutationRoot;

#[juniper::graphql_object(context = Context)]
impl MutationRoot {
    fn session() -> FieldResult<SessionMutations> {
        Ok(SessionMutations {})
    }

    fn account() -> FieldResult<AccountMutations> {
        Ok(AccountMutations {})
    }
}
