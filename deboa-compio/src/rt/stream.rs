use compio_io::util::Splittable;
use cyper_core::HyperStream;

pub enum CompioStream<S>
where
    S: Splittable,
{
    /// A plain TCP connection.
    Plain(HyperStream<S>),

    Tls(HyperStream<S>),
}
