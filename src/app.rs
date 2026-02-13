use std::collections::HashMap;

use crate::actor::Handler;
use crate::{Context, Response};

type RouteHandler = Box<dyn Fn() -> Response>;

pub struct App {
    routes: HashMap<String, RouteHandler>,
}

impl App {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    pub fn register<A, M>(&mut self, path: &str)
    where
        A: Handler<M> + Default + 'static,
        M: Default + 'static,
    {
        let handler = move || {
            let mut actor = A::default();
            let msg = M::default();
            let ctx = Context;
            actor.handle(msg, &ctx)
        };

        self.routes.insert(path.to_string(), Box::new(handler));
    }
    pub fn call(&self, path: &str) -> Option<Response> {
        self.routes.get(path).map(|h| h())
    }
}
