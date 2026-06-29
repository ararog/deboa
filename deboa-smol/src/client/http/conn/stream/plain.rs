use crate::rt::stream::SmolStream;
use deboa::{
    errors::{ConnectionError, DeboaError},
    Result,
};
use smol::net::TcpStream;
use std::net::IpAddr;

pub(crate) async fn create_stream(addr: IpAddr, host: &str, port: u16) -> Result<TcpStream> {
    let tcp_stream = TcpStream::connect((addr, port)).await;
    let tcp_stream = match tcp_stream {
        Ok(tcp_stream) => tcp_stream,
        Err(e) => {
            return Err(DeboaError::Connection(ConnectionError::Tcp {
                host: host.to_string(),
                message: format!("Could not connect to server: {}", e),
            }));
        }
    };

    Ok(tcp_stream)
}

pub(crate) async fn plain_connection(addr: IpAddr, host: &str, port: u16) -> Result<SmolStream> {
    let stream = create_stream(addr, host, port).await?;
    Ok(SmolStream::Plain(stream))
}
