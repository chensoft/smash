use tokio::time;
use tokio::sync::watch;
use tokio::signal::ctrl_c;

use std::sync::LazyLock;

#[macro_export]
macro_rules! spawn {
    ($actor:expr) => {{
        $crate::spawn!($actor, ())
    }};

    ($actor:expr, $arg:expr) => {{
        let (owner, proxy) = $crate::actor::Owner::new($actor, $crate::actor::WORLD.1.clone());
        tokio::spawn(owner.boot($arg));
        proxy
    }};

    ($actor:expr, $arg:expr, $tweak:expr) => {{
        let (owner, proxy) = $crate::actor::Owner::new($actor, $crate::actor::WORLD.1.clone());
        tokio::spawn($tweak(owner.boot($arg)));
        proxy
    }};
}

#[macro_export]
macro_rules! active {
    () => {{
        $crate::actor::ACTIVE.try_with(|v| v.as_ref().downcast_ref::<$crate::Proxy<Self>>().cloned()).ok().flatten().unwrap()
    }};

    ($actor:path) => {{
        $crate::actor::ACTIVE.try_with(|v| v.as_ref().downcast_ref::<$crate::Proxy<$actor>>().cloned()).ok().flatten()
    }};
}

#[macro_export]
macro_rules! run {
    () => {{
        $crate::actor::WORLD.run().await
    }};
}

#[macro_export]
macro_rules! stop {
    () => {{
        $crate::actor::WORLD.stop()
    }};
}

pub static WORLD: LazyLock<World> = LazyLock::new(|| {
    let (tx, rx) = watch::channel(());
    World(tx, rx)
});

pub struct World(pub watch::Sender<()>, pub watch::Receiver<()>);

impl World {
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