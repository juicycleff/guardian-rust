use crate::api::graphql::schema::context::Context;
use crate::dtos::account_dto::AccountResponse;
use crate::dtos::auth_dto::IdentifierRequest;
use crate::services::account_services;
use juniper::FieldResult;

pub struct QueryRoot;

#[juniper::graphql_object(context = Context)]
impl QueryRoot {
    fn apiVersion() -> &str {
        "1.0"
    }

    #[graphql(arguments(identity(description = "identity of the account")))]
    async fn account(context: &Context, identity: String) -> FieldResult<AccountResponse> {
        let rsp =
            account_services::find_account(&context.store, &IdentifierRequest { identity }).await?;
        Ok(rsp)
    }

    #[graphql(arguments(identity(description = "identity of the account")))]
    async fn accountAvailable(context: &Context, identity: String) -> FieldResult<bool> {
        let rsp =
            account_services::find_account(&context.store, &IdentifierRequest { identity }).await;
        match rsp {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}
