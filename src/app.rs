use std::collections::HashMap;

use crate::actor::Handler;
use crate::{Context, ErasedHandler, Response, FromRequest, Request};

use std::marker::PhantomData;

type RouteHandler = Box<dyn ErasedHandler>;

pub struct App {
    routes: HashMap<String, RouteHandler>,
}

struct Route<A, M> {
    actor: A,
    _marker: PhantomData<M>,
}

impl<A, M> ErasedHandler for Route<A, M>
where
    A: Handler<M> + Default + 'static,
    M: FromRequest + 'static,
{
    fn call(&mut self, req: &Request) -> Response {
        let msg = M::from_request(req);
        let ctx = Context;
        self.actor.handle(msg, &ctx)
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
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
        M: FromRequest + 'static,
    {
        let route = Route {
            actor: A::default(),
            _marker: PhantomData,
        };

        self.routes.insert(path.to_string(), Box::new(route));
    }
    pub fn call(&mut self, path: &str, req: &Request) -> Option<Response> {
        self.routes.get_mut(path).map(|e| e.call(req))
    }
}
