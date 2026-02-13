use rex::*;

#[derive(Default)]
struct CounterActor {
    count: usize,
}

#[derive(Default)]
struct Increment;

impl Actor for CounterActor {}

impl Handler<Increment> for CounterActor {
    fn handle(&mut self, _msg: Increment, _ctx: &Context) -> Response {
        self.count += 1;
        Response::text(format!("Count: {}", self.count))
    }
}

fn main() {
    let mut app = App::new();

    app.register::<CounterActor, Increment>("/inc");
    app.call("/inc").unwrap();
    app.call("/inc").unwrap();
    app.call("/inc").unwrap();
    app.call("/inc").unwrap();
    let res = app.call("/inc").unwrap();
    println!("{}", res.body());
}
