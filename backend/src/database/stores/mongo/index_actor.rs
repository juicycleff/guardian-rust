use actix::{Actor, Context};
use actix::prelude::*;
use crate::common::helpers::AppResult;
use mongodb::Database;

/// Define message
#[derive(Message)]
#[rtype(result = "AppResult<()>")]
pub struct IndexMongo {
    pub db: Database
}

// Define actor
pub struct IndexActor;

// Provide Actor implementation for our actor
impl Actor for IndexActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        println!("Actor is alive");
    }

    fn stopped(&mut self, ctx: &mut Context<Self>) {
        println!("Actor is stopped");
    }
}

/// Define handler for `Ping` message
impl Handler<IndexMongo> for IndexActor {
    type Result = AppResult<()>;

    fn handle(&mut self, msg: IndexMongo, ctx: &mut Context<Self>) -> Self::Result {
        println!("Ping received");

        Ok(())
    }
}