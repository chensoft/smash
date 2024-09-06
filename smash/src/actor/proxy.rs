use crate::error::*;
use super::types::*;

use std::sync::LazyLock;
use tokio::sync::{mpsc, oneshot};

pub struct Proxy<A: Actor> {
    mailbox: mpsc::Sender<BoxLetter<A>>,
    sigquit: mpsc::Sender<()>,
    sigkill: mpsc::Sender<()>,
}

impl<A: Actor> Proxy<A> {
    pub fn new(mailbox: mpsc::Sender<BoxLetter<A>>, sigquit: mpsc::Sender<()>, sigkill: mpsc::Sender<()>) -> Self {
        Self { mailbox, sigquit, sigkill }
    }

    pub async fn send<M>(&self, msg: M) -> Result<(), Error>
    where
        A: Handler<M>,
        M: Message,
    {
        Ok(self.mailbox.send(Envelope::new(msg, None)).await?)
    }

    pub async fn tell<M>(&self, msg: M) -> Result<oneshot::Receiver<A::Output>, Error>
    where
        A: Handler<M>,
        M: Message,
    {
        let (snd, rcv) = oneshot::channel();
        self.mailbox.send(Envelope::new(msg, Some(snd))).await?;
        Ok(rcv)
    }

    pub async fn call<M>(&self, msg: M) -> Result<A::Output, Error>
    where
        A: Handler<M>,
        M: Message,
    {
        Ok(self.tell(msg).await?.await?)
    }

    pub fn quit(&self) {
        let _ = self.sigquit.try_send(());
    }

    pub fn kill(&self) {
        let _ = self.sigkill.try_send(());
    }
}

impl<A: Actor> Default for Proxy<A> {
    fn default() -> Self {
        static DUMMY: LazyLock<mpsc::Sender<()>> = LazyLock::new(|| mpsc::channel(1).0);

        Self {
            mailbox: mpsc::channel(1).0,
            sigquit: DUMMY.clone(),
            sigkill: DUMMY.clone(),
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