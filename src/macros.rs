#[cfg(test)]
#[macro_use]
mod test {
    #[macro_export]
    macro_rules! actix_test_cases {
        { $(async fn $name:ident() -> Result<(), $err:ty> $body:block)+ } => {
            $(
                #[test]
                fn $name() {
                    async fn test() -> Result<(), $err> {
                        $body
                    }

                    assert_eq!(actix::System::run(||{
                        use ::futures_util::{future::FutureExt, try_future::TryFutureExt};
                        use ::tokio::prelude::{
                            Future as Future01,
                            FutureExt as Future01Ext,
                        };

                        let f = test().boxed().compat();
                        let f = f.timeout(std::time::Duration::from_secs(5));

                        ::actix::Arbiter::spawn(f.then(|r| {
                            r.unwrap();
                            ::actix::System::current().stop();
                            Ok(())
                        }));
                    }), 0);
                }
            )*
        }
    }
}
