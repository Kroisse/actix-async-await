// async fn
#![feature(async_await, await_macro, futures_api)]

#[macro_use]
mod macros;

mod actor;
mod arbiter;
mod handler;

pub mod prelude;

#[cfg(feature = "actix-web")]
pub mod web;

pub use actor::AsyncContextExt;
pub use arbiter::ArbiterExt;
pub use handler::ResponseStdFuture;
