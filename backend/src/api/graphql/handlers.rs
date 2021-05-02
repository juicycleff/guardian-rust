use std::time::Duration;

use actix_identity::Identity;
use actix_web::web::Data;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use juniper_graphql_ws::ConnectionConfig;

use juniper_actix::subscriptions::subscriptions_handler;
use juniper_actix::{graphiql_handler, graphql_handler, playground_handler};

use crate::api::graphql::schema::context::Context;
use crate::api::graphql::schema::root::Schema;
use crate::database::stores::base_store_trait::BoxedStoreType;

pub async fn graphql(
    req: actix_web::HttpRequest,
    payload: actix_web::web::Payload,
    schema: web::Data<Schema>,
    store: Data<BoxedStoreType>,
    identity: Identity,
) -> Result<HttpResponse, Error> {
    let context = Context::new(store, identity);
    graphql_handler(&schema, &context, req, payload).await
}

pub async fn graphiql_playground() -> Result<HttpResponse, Error> {
    graphiql_handler("/graphql/graph", Some("/graphql/subscriptions")).await
}

pub async fn playground() -> Result<HttpResponse, Error> {
    playground_handler("/graphql/graph", Some("/graphql/subscriptions")).await
}

pub async fn subscriptions(
    req: HttpRequest,
    stream: web::Payload,
    store: Data<BoxedStoreType>,
    schema: web::Data<Schema>,
    identity: Identity,
) -> Result<HttpResponse, actix_web::Error> {
    let context = Context::new(store, identity);
    let schema = schema.into_inner();
    let config = ConnectionConfig::new(context);
    // set the keep alive interval to 15 secs so that it doesn't timeout in playground
    // playground has a hard-coded timeout set to 20 secs
    let config = config.with_keep_alive_interval(Duration::from_secs(15));

    subscriptions_handler(req, stream, schema, config).await
}
