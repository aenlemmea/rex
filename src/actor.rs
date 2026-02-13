use crate::types::{Context, Response};

pub trait Actor {}

pub trait Handler<M>: Actor {
    fn handle(&mut self, msg: M, ctx: &Context) -> Response;
}
