use super::types::*;
use super::proxy::*;

use std::any::Any;
use std::future::Future;

use tokio::sync::{mpsc, watch};
use tokio::task::futures::TaskLocalFuture;

task_local! {
    pub static ACTIVE: Box<dyn Any + Send + 'static>;
}

pub struct Owner<A: Actor> {
    actor: A,
    proxy: Proxy<A>,

    mailbox: mpsc::Receiver<BoxLetter<A>>,
    sigquit: mpsc::Receiver<()>,
    sigkill: mpsc::Receiver<()>,
    sigstop: watch::Receiver<()>,
}

impl<A: Actor> Owner<A> {
    pub fn new(actor: A, sigstop: watch::Receiver<()>) -> (Self, Proxy<A>) {
        let (mailbox_tx, mailbox_rx) = mpsc::channel(1024);
        let (sigquit_tx, sigquit_rx) = mpsc::channel(1);
        let (sigkill_tx, sigkill_rx) = mpsc::channel(1);

        let proxy = Proxy::new(mailbox_tx, sigquit_tx, sigkill_tx);
        (Self { actor, proxy: proxy.clone(), mailbox: mailbox_rx, sigquit: sigquit_rx, sigkill: sigkill_rx, sigstop }, proxy)
    }

    // todo handle panic
    pub fn boot(self, arg: A::Arg) -> TaskLocalFuture<Box<(dyn Any + Send + 'static)>, impl Future<Output = ()>> {
        ACTIVE.scope(Box::new(self.proxy.clone()), self.exec(arg))
    }

    async fn exec(mut self, arg: A::Arg) {
        let mut error = None;
        let mut start = false;

        select! {
            result = self.actor.started(arg) => {
                match result {
                    Ok(_) => start = true,
                    Err(err) => error = Some(err),
                }
            }
            _ = self.sigkill.recv() => {}
            _ = self.sigstop.changed() => {}
        }

        #[allow(clippy::while_immutable_condition)]
        while start {
            let letter = select! {
                letter = self.mailbox.recv() => letter,
                _ = self.sigquit.recv() => {
                    match self.actor.stopping().await {
                        Ok(false) => continue,
                        Ok(true) => break,
                        Err(err) => {
                            error = Some(err);
                            break;
                        }
                    }
                }
                _ = self.sigkill.recv() => break,
                _ = self.sigstop.changed() => break,
            };

            select! {
                finish = Self::poll(&mut self.actor, letter) => {
                    if finish {
                        break;
                    }
                }
                _ = self.sigkill.recv() => break,
                _ = self.sigstop.changed() => break,
            }
        }

        self.actor.stopped(error).await;
    }

    async fn poll(actor: &mut A, letter: Option<BoxLetter<A>>) -> bool {
        match letter {
            Some(mut letter) => {
                letter.handle(actor).await;
                false
            }
            None => true
        }
    }
}