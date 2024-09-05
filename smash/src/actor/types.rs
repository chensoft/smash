use tokio::sync::oneshot;

#[async_trait]
pub trait Actor: Sized + Send + 'static {
    type Arg: Send + 'static;
    type Err: Send + 'static;

    async fn new(arg: Self::Arg) -> Result<Self, Self::Err>;
    async fn quit(&mut self) -> Result<bool, Self::Err> { Ok(true) }
    async fn stop(&mut self, err: Option<Self::Err>);
}

#[async_trait]
pub trait Handler<M: Message> {
    type Output: Send + 'static;

    async fn handle(&mut self, msg: M) -> Self::Output;
}

pub trait Message: Send + 'static {}

#[async_trait]
pub trait Letter<A: Actor> {
    async fn handle(&mut self, act: &mut A);
}

pub type BoxLetter<A> = Box<dyn Letter<A> + Send + 'static>;

pub struct Envelope<A, M>
where
    A: Actor + Handler<M>,
    M: Message,
{
    msg: Option<M>,
    snd: Option<oneshot::Sender<A::Output>>,
}

impl<A, M> Envelope<A, M>
where
    A: Actor + Handler<M>,
    M: Message
{
    pub fn new(msg: M, snd: Option<oneshot::Sender<A::Output>>) -> Box<Self> {
        Box::new(Self { msg: Some(msg), snd })
    }
}

#[async_trait]
impl<A, M> Letter<A> for Envelope<A, M>
where
    A: Actor + Handler<M>,
    M: Message,
{
    async fn handle(&mut self, act: &mut A) {
        let Some(msg) = self.msg.take() else {
            return;
        };

        let ret = act.handle(msg).await;

        if let Some(snd) = self.snd.take() {
            let _ = snd.send(ret);
        }
    }
}