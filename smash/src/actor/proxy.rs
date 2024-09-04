use super::actor::*;

use tokio::sync::{mpsc, watch};

pub struct Proxy<A: Actor> {
    mailbox: mpsc::Sender<A>,
    sigquit: watch::Sender<()>,
    sigkill: watch::Sender<()>,
}

impl<A: Actor> Proxy<A> {
    // pub async fn send(&self, msg: impl Into<A::Msg>) -> Result<(), Error> {
    //     Ok(self.mailbox.as_ref().ok_or(Error::Closed)?.send(Event::Data(msg.into())).await?)
    // }
    //
    // pub async fn call(&self) {
    //     todo!()
    // }
    //
    // pub async fn quit(&self) -> Result<(), Error> {
    //     match &self.mailbox {
    //         Some(mailbox) => Ok(mailbox.send(Event::Quit).await?),
    //         None => Ok(())
    //     }
    // }
    //
    // pub async fn kill(&self) -> Result<(), Error> {
    //     self.sigkill.as_ref().map(|sig| sig.send(()));
    //     Ok(())
    // }
}