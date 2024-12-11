use tokio::sync::oneshot;

#[async_trait]
#[allow(unused_variables)]
pub trait Actor: Sized + Send + 'static {
    type Arg: Send + 'static;
    type Err: Send + 'static;

    async fn started(&mut self, arg: Self::Arg) -> Result<(), Self::Err> { Ok(()) }
    async fn stopping(&mut self, err: &Option<Self::Err>) -> Result<bool, Self::Err> { Ok(true) }
    async fn stopped(&mut self, err: &Option<Self::Err>) {}
}

#[async_trait]
pub trait Handler<M: Send + 'static> {
    type Output: Send + 'static;

    async fn handle(&mut self, msg: M) -> Self::Output;

    #[inline]
    async fn filter(&mut self, out: Self::Output) -> Self::Output {
        out
    }
}

#[async_trait]
pub trait Letter<A: Actor> {
    async fn handle(&mut self, act: &mut A);
}

pub type BoxLetter<A> = Box<dyn Letter<A> + Send + 'static>;

pub struct Envelope<A, M>
where
    A: Actor + Handler<M>,
    M: Send + 'static,
{
    msg: Option<M>,
    snd: Option<oneshot::Sender<A::Output>>,
}

impl<A, M> Envelope<A, M>
where
    A: Actor + Handler<M>,
    M: Send + 'static
{
    pub fn new(msg: M, snd: Option<oneshot::Sender<A::Output>>) -> Box<Self> {
        Box::new(Self { msg: Some(msg), snd })
    }
}

#[async_trait]
impl<A, M> Letter<A> for Envelope<A, M>
where
    A: Actor + Handler<M>,
    M: Send + 'static,
{
    async fn handle(&mut self, act: &mut A) {
        let Some(msg) = self.msg.take() else {
            return;
        };

        let out = act.handle(msg).await;
        let ret = act.filter(out).await;

        if let Some(snd) = self.snd.take() {
            let _ = snd.send(ret);
        }
    }
}