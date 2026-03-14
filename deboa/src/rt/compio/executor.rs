use hyper::rt::Executor;
use std::future::Future;

#[non_exhaustive]
#[derive(Default, Debug, Clone)]
pub struct CompioExecutor {}

impl<Fut> Executor<Fut> for CompioExecutor
where
    Fut: Future + Send + 'static,
    Fut::Output: Send + 'static,
{
    fn execute(&self, fut: Fut) {
        compio::runtime::spawn(fut).detach();
    }
}

impl CompioExecutor {
    pub fn new() -> Self {
        Self {}
    }
}
