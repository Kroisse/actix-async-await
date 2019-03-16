use std::future::Future;
use std::marker::Unpin;

use ::actix::prelude::*;
use futures_util::{future::FutureExt, try_future::TryFutureExt};

/// Additional methods for spawning futures.
pub trait ArbiterExt {
    /// Executes a future on the current thread.
    fn spawn_async<F>(fut: F)
    where
        F: Future<Output = ()> + Unpin + 'static;
}

impl ArbiterExt for Arbiter {
    fn spawn_async<F>(fut: F)
    where
        F: Future<Output = ()> + Unpin + 'static,
    {
        Self::spawn(fut.unit_error().compat());
    }
}
