use smash::{Actor, Event};

/// A Message
struct Ping(&'static str);

/// The Actor
// #[derive(Actor)]
struct Echo {
    count: usize,
}

impl Actor for Echo {
    type Arg = ();
    type Err = ();

    async fn new(arg: Self::Arg) -> Result<Self, Self::Err> {
        todo!()
    }

    async fn stop(&mut self, err: Option<Self::Err>) {
        todo!()
    }
}

impl Event<Ping> for Echo {
    type Output = ();

    async fn handle(&mut self, msg: Ping) -> Self::Output {
        todo!()
    }
}

#[tokio::main]
async fn main() {
    let mut echo = Echo { count: 0 };
    echo.handle(Ping("abc")).await;

//     let echo = smash::spawn!(Echo, 0);
//     let pong = echo.call(Ping("hi")).await;
//
//     assert_eq!(pong, "hi");
//
//     smash::run();
}