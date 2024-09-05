use smash::*;
use async_trait::*;

/// A Message
struct Ping(&'static str);

impl Message for Ping {
}

/// The Actor
struct Echo {
    count: usize,
}

#[async_trait]
impl Actor for Echo {
    type Arg = usize;
    type Err = ();

    async fn new(arg: Self::Arg) -> Result<Self, Self::Err> {
        Ok(Self { count: arg })
    }

    async fn stop(&mut self, err: Option<Self::Err>) {
        println!("stop");
    }
}

#[async_trait]
impl Handler<Ping> for Echo {
    type Output = ();

    async fn handle(&mut self, msg: Ping) -> Self::Output {
        println!("ping");
    }
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let mut echo = smash::spawn!(Echo, 111)?;
    let _ = echo.send(Ping("abc")).await;

//     let echo = smash::spawn!(Echo, 0);
//     let pong = echo.call(Ping("hi")).await;
//
//     assert_eq!(pong, "hi");

    Ok(smash::run!())
}