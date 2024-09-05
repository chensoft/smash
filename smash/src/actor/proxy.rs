use crate::error::*;
use super::actor::*;

use std::marker::PhantomData;
use tokio::sync::{mpsc, watch, oneshot};

pub struct Proxy<A: Actor> {
    mailbox: mpsc::Sender<BoxLetter<A>>,
    sigquit: watch::Sender<()>, // todo combine to control
    sigkill: watch::Sender<()>,

    _marker: PhantomData<A>
}

impl<A: Actor> Proxy<A> {
    pub fn new(mailbox: mpsc::Sender<BoxLetter<A>>, sigquit: watch::Sender<()>, sigkill: watch::Sender<()>) -> Self {
        Self { mailbox, sigquit, sigkill, _marker: PhantomData::default() }
    }

    pub async fn send<M>(&self, msg: M) -> Result<(), Error>
    where
        A: Handler<M>,
        M: Message,
    {
        Ok(self.mailbox.send(Envelope::new(msg, None)).await?)
    }

    pub async fn call<M>(&self, msg: M) -> Result<A::Output, Error>
    where
        A: Handler<M>,
        M: Message,
    {
        let (snd, rcv) = oneshot::channel();
        self.mailbox.send(Envelope::new(msg, Some(snd))).await?;
        Ok(rcv.await?)
    }

    pub fn quit(&self) -> Result<(), ()> {
        //     match &self.mailbox {
        //         Some(mailbox) => Ok(mailbox.send(Event::Quit).await?),
        //         None => Ok(())
        //     }
        todo!()
    }

    pub fn kill(&self) -> Result<(), ()> {
        //     self.sigkill.as_ref().map(|sig| sig.send(()));
        //     Ok(())
        todo!()
    }
}

impl<A: Actor> Clone for Proxy<A> {
    fn clone(&self) -> Self {
        Self {
            mailbox: self.mailbox.clone(),
            sigquit: self.sigquit.clone(),
            sigkill: self.sigkill.clone(),
            _marker: self._marker.clone(),
        }
    }
}