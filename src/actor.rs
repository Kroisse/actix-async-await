use std::marker::Unpin;

use ::actix::prelude::*;
use futures_core::{Stream, TryStream};
use futures_util::{stream::StreamExt, try_stream::TryStreamExt};

pub trait AsyncContextExt<A>: AsyncContext<A>
where
    A: Actor<Context = Self>,
{
    /// This method register stream to an actor context and allows
    /// to handle [Stream] in similar way as normal actor messages.
    fn add_stream_03<S>(&mut self, fut: S) -> SpawnHandle
    where
        S: TryStream + Unpin + 'static,
        A: StreamHandler<S::Ok, S::Error>,
    {
        self.add_stream(fut.compat())
    }

    fn add_message_stream_03<S>(&mut self, fut: S)
    where
        S: Stream + Unpin + 'static,
        S::Item: Message,
        A: Handler<S::Item>,
    {
        self.add_message_stream(fut.map(Ok).compat())
    }
}

impl<A, C> AsyncContextExt<A> for C
where
    A: Actor<Context = C>,
    C: AsyncContext<A>,
{
}
