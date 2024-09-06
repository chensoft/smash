use smash::{Actor, Handler, Message};
use async_trait::*;

/// A Message
struct Ping(&'static str);

impl Message for Ping {}

/// The Actor
struct Echo {
    id: i32,
}

#[async_trait]
impl Actor for Echo {
    type Arg = i32;
    type Err = anyhow::Error;

    async fn new(arg: Self::Arg) -> Result<Self, Self::Err> {
        Ok(Self { id: arg })
    }

    async fn stopped(&mut self, err: Option<Self::Err>) {
        println!("stop {err:?}");
    }
}

#[async_trait]
impl Handler<Ping> for Echo {
    type Output = String;

    async fn handle(&mut self, msg: Ping) -> Self::Output {
        println!("{} {}", self.id, msg.0);
        msg.0.to_string()
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let echo = smash::spawn!(Echo, 1)?;
    let pong = echo.call(Ping("hi")).await?;

    assert_eq!(pong, "hi");

    Ok(smash::run!())
}