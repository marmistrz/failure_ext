use futures::Future;
use std::fmt::Display;

/// Extension methods for `trait Future`.
pub trait FutureExt<F>
where
    F: Future,
{
    /// Wraps the error type in a context type.
    ///
    /// # Examples
    ///
    /// ```
    /// # use futures::prelude::*;
    /// # use failure::err_msg;
    /// # use failure_ext::FutureExt;
    /// fn get_future() -> impl Future<Item = (), Error = failure::Error> {
    ///     futures::future::err(err_msg("abcd"))
    /// }
    ///
    /// let x = get_future()
    ///     .context(format!("An error occured"))
    ///     .wait()
    ///     .unwrap_err();
    ///
    /// let x = format!("{}", x);
    /// assert_eq!(x, "An error occured");
    /// ```
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
