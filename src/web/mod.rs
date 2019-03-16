mod with;

use actix_web::{dev::Route, Error, FromRequest, Responder};
use futures_core::TryFuture;

use self::with::WithAsyncFactory;

pub trait RouteExt<S> {
    fn with_std_async<T, F, R>(&mut self, handler: F)
    where
        F: WithAsyncFactory<T, S, R>,
        R: TryFuture + 'static,
        R::Ok: Responder + 'static,
        R::Error: Into<Error>,
        T: FromRequest<S> + 'static;

    fn with_std_async_config<T, F, R>(&mut self, handler: F, cfg: impl FnOnce(&mut T::Config))
    where
        F: WithAsyncFactory<T, S, R>,
        R: TryFuture + 'static,
        R::Ok: Responder + 'static,
        R::Error: Into<Error>,
        T: FromRequest<S> + 'static;
}

impl<S: 'static> RouteExt<S> for Route<S> {
    fn with_std_async<T, F, R>(&mut self, handler: F)
    where
        F: WithAsyncFactory<T, S, R>,
        R: TryFuture + 'static,
        R::Ok: Responder + 'static,
        R::Error: Into<Error>,
        T: FromRequest<S> + 'static,
    {
        self.h(handler.create())
    }

    fn with_std_async_config<T, F, R>(&mut self, handler: F, cfg: impl FnOnce(&mut T::Config))
    where
        F: WithAsyncFactory<T, S, R>,
        R: TryFuture + 'static,
        R::Ok: Responder + 'static,
        R::Error: Into<Error>,
        T: FromRequest<S> + 'static,
    {
        let mut extractor_cfg = <T::Config as Default>::default();
        cfg(&mut extractor_cfg);
        self.h(handler.create_with_config(extractor_cfg))
    }
}
