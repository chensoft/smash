use super::types::*;
use super::proxy::*;

use std::any::Any;
use tokio::sync::{mpsc, watch};

task_local! {
    pub static RUNNING: Box<dyn Any + Send + 'static>;
}

pub struct Owner<A: Actor> {
    actor: A,
    mailbox: mpsc::Receiver<BoxLetter<A>>,
    sigquit: mpsc::Receiver<()>,
    sigkill: mpsc::Receiver<()>,
    sigstop: watch::Receiver<()>,
}

impl<A: Actor> Owner<A> {
    pub async fn new(arg: A::Arg, sigstop: watch::Receiver<()>) -> Result<Proxy<A>, A::Err> {
        let (mailbox_tx, mailbox_rx) = mpsc::channel(1024);
        let (sigquit_tx, sigquit_rx) = mpsc::channel(1);
        let (sigkill_tx, sigkill_rx) = mpsc::channel(1);

        let proxy = Proxy::new(mailbox_tx, sigquit_tx, sigkill_tx);

        // todo handle panic
        tokio::spawn(RUNNING.scope(Box::new(proxy.clone()), async move {
            let actor = <A as Actor>::new(arg).await?;
            let owner = Self { actor, mailbox: mailbox_rx, sigquit: sigquit_rx, sigkill: sigkill_rx, sigstop };
            Ok::<_, A::Err>(owner.run().await)
        }));

        Ok(proxy)
    }

    pub async fn run(mut self) {
        let mut error = None;

        loop {
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