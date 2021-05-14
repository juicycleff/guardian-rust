use juniper::FieldResult;

use crate::api::graphql::schema::context::Context;
use crate::api::services;
use crate::common::auth::utils::{create_jwt, PrivateClaim};
use crate::data::dtos::account_dto::PostSessionResponse;
use crate::data::dtos::auth_dto::PostSessionRequest;

pub struct SessionMutations;

#[juniper::graphql_object(context = Context)]
impl SessionMutations {
    async fn create(
        context: &Context,
        input: PostSessionRequest,
    ) -> FieldResult<PostSessionResponse> {
        let acct = services::session_service::create_session(&context.store, &input).await?;
        // id.remember(acct.id.clone());

        // create jwt token
        let pc = PrivateClaim::new(acct.id, acct.email, acct.username, acct.mobile);
        let token = create_jwt(pc).unwrap_or_default();

        Ok(PostSessionResponse {
            id_token: Option::from(token),
        })
    }

    async fn delete(context: &Context) -> FieldResult<bool> {
        // let acct = services::session_service::create_session(&context.store, &input).await?;
        // id.remember(acct.id.clone());
        Ok(false)
    }

    async fn refresh(context: &Context) -> FieldResult<bool> {
        // let acct = services::session_service::create_session(&context.store, &input).await?;
        // id.remember(acct.id.clone());
        Ok(false)
    }
}
