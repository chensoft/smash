use tokio::time;
use tokio::sync::watch;
use tokio::signal::ctrl_c;

use std::sync::LazyLock;

#[macro_export]
macro_rules! spawn {
    ($actor:ident, $arg:expr) => {{
        $crate::spawn!($actor, $arg, ())
    }};

    ($actor:ident, $arg:expr, $udata:expr) => {{
        $crate::actor::Owner::<$actor>::new($arg, $crate::actor::AGENT.1.clone()).await
    }};
}

#[macro_export]
macro_rules! running {
    () => {{
        $crate::actor::RUNNING.try_with(|v| v.as_ref().downcast_ref::<$crate::Proxy<Self>>().cloned()).ok().flatten().unwrap()
    }};

    ($actor:ident) => {{
        $crate::actor::RUNNING.try_with(|v| v.as_ref().downcast_ref::<$crate::Proxy<$actor>>().cloned()).ok().flatten()
    }};
}

#[macro_export]
macro_rules! run {
    () => {{
        $crate::actor::AGENT.run().await
    }};
}

#[macro_export]
macro_rules! stop {
    () => {{
        $crate::actor::AGENT.stop()
    }};
}

pub static AGENT: LazyLock<Agent> = LazyLock::new(|| {
    let (tx, rx) = watch::channel(());
    Agent(tx, rx)
});

pub struct Agent(pub watch::Sender<()>, pub watch::Receiver<()>);

impl Agent {
    pub async fn run(&self) {
        let mut check = time::interval(time::Duration::from_secs(1));
        let mut twice = false;

        loop {
            select! {
                _ = check.tick() => {
                    if self.0.receiver_count() <= 1 { // todo 1?
                        break;
                    }
                }
                _ = ctrl_c() => {
                    if twice {
                        break;
                    }

                    self.stop();

                    twice = true;
                }
            }
        }
    }

    pub fn stop(&self) {
        let _ = self.0.send(());
    }
}