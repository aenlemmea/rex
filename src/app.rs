use std::collections::HashMap;

use crate::actor::Handler;
use crate::{Context, ErasedHandler, Response};
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
    M: Default + 'static,
{
    fn call(&mut self) -> Response {
        let msg = M::default();
        let ctx = Context;
        self.actor.handle(msg, &ctx)
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
        M: Default + 'static,
    {
        let route = Route {
            actor: A::default(),
            _marker: PhantomData,
        };

        self.routes.insert(path.to_string(), Box::new(route));
    }
    pub fn call(&mut self, path: &str) -> Option<Response> {
        self.routes.get_mut(path).map(|e| e.call())
    }
}
