use rex::*;

#[derive(Default)]
struct Hello;

#[derive(Default)]
struct HelloActor;

impl Actor for HelloActor {}

impl Handler<Hello> for HelloActor {
    fn handle(&mut self, _msg: Hello, _ctx: &Context) -> Response {
        Response::text("Hello From Actor!")
    }
}

impl FromRequest for Hello {
    fn from_request(req: &Request) -> Self {
        req.logf();
        Hello
    }
}


fn main() {
    let mut app = App::new();

    let req = Request::new(
        "GET".to_string(),
        "NULL".to_string(),
        "NULL".to_string(),
        "/hello".to_string(),
        "NULL".to_string(),
    );

    app.register::<HelloActor, Hello>("/hello");
    let res = app.call("/hello", &req).unwrap();
    println!("{}", res.body());
}
