use crate::error::*;
use super::actor::*;

use tokio::sync::{mpsc, watch};

pub struct Proxy<A: Actor> {
    mailbox: mpsc::Sender<()>,
    sigquit: watch::Sender<()>,
    sigkill: watch::Sender<()>,

    _x: std::marker::PhantomData<A>
}

impl<A: Actor> Proxy<A> {
    pub fn new(mailbox: mpsc::Sender<()>, sigquit: watch::Sender<()>, sigkill: watch::Sender<()>) -> Self {
        Self { mailbox, sigquit, sigkill, _x: Default::default() }
    }

    // pub async fn send(&self, msg: impl Into<A::Msg>) -> Result<(), Error> {
    //     Ok(self.mailbox.as_ref().ok_or(Error::Closed)?.send(Event::Data(msg.into())).await?)
    // }

    pub async fn call(&self) {
        todo!()
    }

    pub fn quit(&self) -> Result<(), Error> {
        //     match &self.mailbox {
        //         Some(mailbox) => Ok(mailbox.send(Event::Quit).await?),
        //         None => Ok(())
        //     }
        todo!()
    }

    pub fn kill(&self) -> Result<(), Error> {
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
            _x: Default::default(), // todo
        }
    }
}