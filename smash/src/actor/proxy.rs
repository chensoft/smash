use crate::error::*;
use super::types::*;

use tokio::sync::{mpsc, oneshot};

pub struct Proxy<A: Actor> {
    mailbox: mpsc::Sender<BoxLetter<A>>,
    sigquit: mpsc::Sender<Option<A::Err>>,
    sigkill: mpsc::Sender<Option<A::Err>>,
}

impl<A: Actor> Proxy<A> {
    pub fn new(mailbox: mpsc::Sender<BoxLetter<A>>, sigquit: mpsc::Sender<Option<A::Err>>, sigkill: mpsc::Sender<Option<A::Err>>) -> Self {
        Self { mailbox, sigquit, sigkill }
    }

    pub async fn send<M>(&self, msg: M) -> Result<(), Error>
    where
        A: Handler<M>,
        M: Send + 'static,
    {
        Ok(self.mailbox.send(Envelope::new(msg, None)).await?)
    }

    pub async fn tell<M>(&self, msg: M) -> Result<oneshot::Receiver<A::Output>, Error>
    where
        A: Handler<M>,
        M: Send + 'static,
    {
        let (snd, rcv) = oneshot::channel();
        self.mailbox.send(Envelope::new(msg, Some(snd))).await?;
        Ok(rcv)
    }

    pub async fn call<M>(&self, msg: M) -> Result<A::Output, Error>
    where
        A: Handler<M>,
        M: Send + 'static,
    {
        Ok(self.tell(msg).await?.await?)
    }

    pub fn quit(&self, err: Option<A::Err>) {
        let _ = self.sigquit.try_send(err);
    }

    pub fn kill(&self, err: Option<A::Err>) {
        let _ = self.sigkill.try_send(err);
    }
}

impl<A: Actor> Default for Proxy<A> {
    fn default() -> Self {
        let dummy = mpsc::channel::<Option<A::Err>>(1).0;

        Self {
            mailbox: mpsc::channel(1).0,
            sigquit: dummy.clone(),
            sigkill: dummy.clone(),
        }
    }
}

impl<A: Actor> Clone for Proxy<A> {
    fn clone(&self) -> Self {
        Self {
            mailbox: self.mailbox.clone(),
            sigquit: self.sigquit.clone(),
            sigkill: self.sigkill.clone(),
        }
    }
}