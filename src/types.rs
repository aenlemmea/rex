pub struct Context;

pub struct Response {
    body: String,
}

impl Response {
    pub fn text(body: impl Into<String>) -> Self {
        Self { body: body.into() }
    }

    pub fn body(&self) -> &str {
        &self.body
    }
}
