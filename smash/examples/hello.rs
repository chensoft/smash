/// A Message
struct Ping(&'static str);

/// The Actor
// #[derive(Actor)]
struct Echo {
    count: usize,
}

impl Echo {
    async fn new(xxx: (i32, i32)) -> Self { todo!() }
}

// trait Event<M> {
//     type Output;
// 
//     fn handle(&self, msg: T) -> Self::Output;
// }
// 
// struct Init;
// struct Quit;
// struct Stop;

// trait Actor {
//     type Arg;
//     type Err;
// 
//     async fn new(arg: Self::Arg) -> Result<Self, Self::Err> { todo!() }
//     async fn init() -> Result<(), Self::Err> { Ok(()) }
//     async fn quit() -> Result<bool, Self::Err> { Ok(true) }
//     async fn stop() {}
// }

#[tokio::main]
async fn main() {
    let kind = 1;
    let args = b"xxx".as_ref();
    


    if kind == 1 {
        let x = Echo::new(bincode::deserialize(args).unwrap()).await;
    }
    
//     let echo = smash::spawn!(Echo, 0);
//     let pong = echo.call(Ping("hi")).await;
//
//     assert_eq!(pong, "hi");
//
//     smash::run();
}