pub struct Context;

pub struct Response {
    body: String,
}

pub struct Request {
    what: String, // GET, PUT, POST, etc.
    query: String,
    body: String,
    path: String,
    params: String,
}

impl Request {
    pub fn new(
        what: String, // GET, PUT, POST, etc.
        query: String,
        body: String,
        path: String,
        params: String,  
    ) -> Self {
        Self { what, query, body, path, params}
    }

    pub fn logf(&self) {
        println!("{} hit from {}\nWith Q: {}\nB: {}\nP: {}", self.what, self.path, self.query, self.body, self.params)
    }
}

pub trait FromRequest {
    fn from_request(req: &Request) -> Self;
}

impl Response {
    pub fn text(body: impl Into<String>) -> Self {
        Self { body: body.into() }
    }

    pub fn body(&self) -> &str {
        &self.body
    }
}
