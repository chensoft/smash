// /// A Message
// struct Ping(&'static str);
//
// /// The Actor
// struct Echo {
//     count: usize,
// }
//
// impl Actor for Echo {
//     type Arg;
//     type Err;
//
//     async fn new(arg: Self::Arg) -> Result<Self, Self::Err> { todo!() }
//     async fn started() -> Result<(), Self::Err> {}
//     async fn quitting() -> Result<bool, Self::Err> { Ok(true) }
//     async fn stopped() {}
// }
//
#[tokio::main]
async fn main() {
//     let echo = smash::spawn!(Echo, 0);
//     let pong = echo.call(Ping("hi")).await;
//
//     assert_eq!(pong, "hi");
//
//     smash::run();
}