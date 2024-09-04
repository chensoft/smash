use smash::{Actor, Event, Proxy};

/// A Message
struct Ping(&'static str);

/// The Actor
// #[derive(Actor)]
struct Echo {
    count: usize,
}

impl Actor for Echo {
    type Arg = i32;
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
async fn main() -> Result<(), ()> {
    let mut echo = smash::spawn!(Echo, 111)?;
    // echo.send(Ping("abc")).await?;

//     let echo = smash::spawn!(Echo, 0);
//     let pong = echo.call(Ping("hi")).await;
//
//     assert_eq!(pong, "hi");

    Ok(smash::run!())
}