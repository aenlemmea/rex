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

fn main() {
    let mut app = App::new();

    app.register::<HelloActor, Hello>("/hello");
    let res = app.call("/hello").unwrap();
    println!("{}", res.body());
}
