use juniper::FieldResult;

use crate::api::graphql::schema::context::Context;
use crate::common::auth::utils::{create_jwt, PrivateClaim};
use crate::dtos::account_dto::PostAccountResponse;
use crate::dtos::auth_dto::PostAccountRequest;
use crate::services;

pub struct AccountMutations;

#[juniper::graphql_object(context = Context)]
impl AccountMutations {
    async fn create(
        context: &Context,
        input: PostAccountRequest,
    ) -> FieldResult<PostAccountResponse> {
        let acct = services::account_services::create_account(&context.store, &input).await?;
        // id.remember(acct.id.clone());

        // create jwt token
        let pc = PrivateClaim::new(acct.id, acct.email, acct.username, acct.mobile);
        let token = create_jwt(pc).unwrap_or_default();

        Ok(PostAccountResponse {
            id_token: Option::from(token),
        })
    }
}
