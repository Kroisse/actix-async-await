// async fn
#![feature(async_await, await_macro, futures_api)]

#[macro_use]
mod macros;

mod actor;
mod arbiter;
mod constants;
mod handler;

pub mod prelude;

pub use actor::AsyncContextExt;
pub use arbiter::ArbiterExt;
pub use handler::ResponseStdFuture;
