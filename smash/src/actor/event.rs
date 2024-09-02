#[allow(async_fn_in_trait)]
pub trait Event<M> {
    type Output;

    async fn handle(&mut self, msg: M) -> Self::Output;
}