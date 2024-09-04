use super::actor::*;
use super::proxy::*;

use tokio::sync::{mpsc, watch};

pub struct Owner<A: Actor> {
    actor: A,
    mailbox: mpsc::Receiver<()>,
    sigstop: watch::Receiver<()>,
    sigquit: watch::Receiver<()>,
    sigkill: watch::Receiver<()>,
}

impl<A: Actor> Owner<A> {
    pub async fn new(arg: A::Arg, stop: watch::Receiver<()>) -> Result<Proxy<A>, A::Err> {
        todo!()
    }
}