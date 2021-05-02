use juniper::FieldResult;

use crate::api::graphql::schema::context::Context;
use crate::common::auth::utils::{create_jwt, PrivateClaim};
use crate::dtos::account_dto::PostSessionResponse;
use crate::dtos::auth_dto::PostSessionRequest;
use crate::services;

pub struct SessionMutations;

#[juniper::graphql_object(context = Context)]
impl SessionMutations {
    async fn create(
        context: &Context,
        input: PostSessionRequest,
    ) -> FieldResult<PostSessionResponse> {
        let acct = services::account_services::login_account(&context.store, &input).await?;
        // id.remember(acct.id.clone());

        // create jwt token
        let pc = PrivateClaim::new(acct.id, acct.email, acct.username, acct.mobile);
        let token = create_jwt(pc).unwrap_or_default();

        Ok(PostSessionResponse {
            id_token: Option::from(token),
        })
    }
}
