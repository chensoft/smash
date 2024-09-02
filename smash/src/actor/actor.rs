#[allow(async_fn_in_trait)]
pub trait Actor: Sized {
    type Arg;
    type Err;

    async fn new(arg: Self::Arg) -> Result<Self, Self::Err>;
    async fn init(&mut self) -> Result<(), Self::Err> { Ok(()) }
    async fn quit(&mut self) -> Result<bool, Self::Err> { Ok(true) }
    async fn stop(&mut self, err: Option<Self::Err>);
}