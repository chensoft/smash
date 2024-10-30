mod types;
mod proxy;
mod owner;
mod world;

pub use types::*;
pub use proxy::*;
pub use owner::*;
pub use world::*;

pub async fn join() {
    WORLD.join().await
}

pub async fn stop() {
    WORLD.stop().await
}