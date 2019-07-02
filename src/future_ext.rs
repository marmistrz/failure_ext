use futures::Future;
use std::fmt::Display;

pub trait FutureExt<F>
where
    F: Future,
{
    fn context<D>(self, context: D) -> Box<dyn Future<Item = F::Item, Error = failure::Error>>
    where
        D: Display + Send + Sync + 'static;
}

impl<F> FutureExt<F> for F
where
    F: Future + 'static,
    F::Error: Into<failure::Error>,
{
    fn context<D>(self, context: D) -> Box<dyn Future<Item = F::Item, Error = failure::Error>>
    where
        D: Display + Send + Sync + 'static,
    {
        Box::new(self.map_err(|e| e.into().context(context).into()))
    }
}
