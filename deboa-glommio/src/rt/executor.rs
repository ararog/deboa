use hyper::rt::Executor;
use std::future::Future;

#[non_exhaustive]
#[derive(Default, Debug, Clone)]
pub(crate) struct GlommioExecutor {}

// **No `Send` bound, and that is the whole point.** hyper's `Executor` trait
// does not require one; the smol and tokio bindings add it because their
// spawns do. glommio's `spawn_local` keeps the future on this core, so the
// per-core binding can implement the same trait more permissively.
impl<Fut> Executor<Fut> for GlommioExecutor
where
    Fut: Future + 'static,
    Fut::Output: 'static,
{
    fn execute(&self, fut: Fut) {
        glommio::spawn_local(fut).detach();
    }
}

impl GlommioExecutor {
    pub fn new() -> Self {
        Self {}
    }
}
