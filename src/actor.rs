use crate::types::{Context, Response};
use crate::Request;

pub trait Actor {}

/*
    TODO:
        Multiple Messages Per Actor
        or 
        Multiple Actors Per Route

        Concurrency!
*/

pub trait ErasedHandler {
    fn call(&mut self, req: &Request) -> Response;
}

pub trait Handler<M>: Actor {
    fn handle(&mut self, msg: M, ctx: &Context) -> Response;
}
