use std::marker::PhantomData;
use std::rc::Rc;

use actix_web::{
    dev::{AsyncResult, Handler},
    *,
};
use futures_core::TryFuture;
use futures_util::{compat::Future01CompatExt, FutureExt, TryFutureExt};

#[doc(hidden)]
pub trait WithAsyncFactory<T, S, R>: 'static
where
    T: FromRequest<S>,
    R: TryFuture,
{
    fn create(self) -> WithAsync<T, S, R>;

    fn create_with_config(self, cfg: T::Config) -> WithAsync<T, S, R>;
}

#[doc(hidden)]
pub struct WithAsync<T, S, R>
where
    T: FromRequest<S>,
    S: 'static,
{
    hnd: Rc<dyn Fn(T) -> R>,
    cfg: Rc<T::Config>,
    _s: PhantomData<S>,
}

impl<T, S, R> WithAsync<T, S, R>
where
    T: FromRequest<S>,
    S: 'static,
{
    pub(crate) fn new(f: impl Fn(T) -> R + 'static, cfg: T::Config) -> Self {
        WithAsync {
            cfg: Rc::new(cfg),
            hnd: Rc::new(f),
            _s: PhantomData,
        }
    }
}

impl<T, S, R> Handler<S> for WithAsync<T, S, R>
where
    R: TryFuture + 'static,
    R::Ok: Responder + 'static,
    R::Error: Into<Error>,
    T: FromRequest<S> + 'static,
    S: 'static,
{
    type Result = AsyncResult<HttpResponse>;

    fn handle(&self, req: &HttpRequest<S>) -> Self::Result {
        let req = req.clone();
        let cfg = Rc::clone(&self.cfg);
        let hnd = Rc::clone(&self.hnd);
        let fut = async move {
            let item = await!(T::from_request(&req, &*cfg).into().compat())?;
            let r = await!((hnd)(item).into_future()).map_err(Into::into)?;
            let res = r.respond_to(&req).map_err(Into::into)?;
            await!(res.into().compat())
        };
        AsyncResult::future(Box::new(fut.boxed().compat()))
    }
}

macro_rules! with_async_factory_tuple ({$(($n:tt, $T:ident)),+} => {
    impl<$($T,)+ State, Func, Res> WithAsyncFactory<($($T,)+), State, Res> for Func
    where
        Func: Fn($($T,)+) -> Res + 'static,
        $($T: FromRequest<State> + 'static,)+
        Res: TryFuture,
        Res::Ok: Responder + 'static,
        Res::Error: Into<Error>,
        State: 'static,
    {
        fn create(self) -> WithAsync<($($T,)+), State, Res> {
            WithAsync::new(move |($($n,)+)| (self)($($n,)+), ($($T::Config::default(),)+))
        }

        fn create_with_config(self, cfg: ($($T::Config,)+)) -> WithAsync<($($T,)+), State, Res> {
            WithAsync::new(move |($($n,)+)| (self)($($n,)+), cfg)
        }
    }
});

with_async_factory_tuple!((a, A));
with_async_factory_tuple!((a, A), (b, B));
with_async_factory_tuple!((a, A), (b, B), (c, C));
with_async_factory_tuple!((a, A), (b, B), (c, C), (d, D));
with_async_factory_tuple!((a, A), (b, B), (c, C), (d, D), (e, E));
with_async_factory_tuple!((a, A), (b, B), (c, C), (d, D), (e, E), (f, F));
with_async_factory_tuple!((a, A), (b, B), (c, C), (d, D), (e, E), (f, F), (g, G));
with_async_factory_tuple!(
    (a, A),
    (b, B),
    (c, C),
    (d, D),
    (e, E),
    (f, F),
    (g, G),
    (h, H)
);
with_async_factory_tuple!(
    (a, A),
    (b, B),
    (c, C),
    (d, D),
    (e, E),
    (f, F),
    (g, G),
    (h, H),
    (i, I)
);
