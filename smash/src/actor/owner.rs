use super::actor::*;
use super::proxy::*;

use std::any::Any;
use tokio::sync::{mpsc, watch};

task_local! {
    pub static RUNNING: Box<dyn Any>;
}

pub struct Owner<A: Actor> {
    actor: A,
    mailbox: mpsc::Receiver<()>,
    sigstop: watch::Receiver<()>,
    sigquit: watch::Receiver<()>,
    sigkill: watch::Receiver<()>,
}

impl<A: Actor> Owner<A> {
    pub async fn new(arg: A::Arg, sigstop: watch::Receiver<()>) -> Result<Proxy<A>, A::Err> {
        let (mailbox_tx, mailbox_rx) = mpsc::channel(1024);
        let (sigquit_tx, sigquit_rx) = watch::channel(());
        let (sigkill_tx, sigkill_rx) = watch::channel(());

        let proxy = Proxy::new(mailbox_tx, sigquit_tx, sigkill_tx);

        // todo handle panic
        RUNNING.scope(Box::new(proxy.clone()), async move {
            let actor = <A as Actor>::new(arg).await?;
            let owner = Self { actor, mailbox: mailbox_rx, sigstop, sigquit: sigquit_rx, sigkill: sigkill_rx };
            Ok::<_, A::Err>(owner.run().await)
        });

        Ok(proxy)
    }

    pub async fn run(mut self) {

    }
}