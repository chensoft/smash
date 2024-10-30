mod types;
mod proxy;
mod owner;
mod world;

pub use types::*;
pub use proxy::*;
pub use owner::*;
pub use world::*;

pub async fn wait() {
    WORLD.wait().await
}

pub async fn stop() {
    WORLD.stop().await
}