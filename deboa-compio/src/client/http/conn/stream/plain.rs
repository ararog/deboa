use compio::net::TcpStream;
use cyper_core::HyperStream;
use deboa::{
    errors::{ConnectionError, DeboaError},
    Result,
};
use std::net::IpAddr;

pub async fn create_stream(ip: IpAddr, host: &str, port: u16) -> Result<TcpStream> {
    let tcp_stream = TcpStream::connect((ip, port)).await;
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

pub(crate) async fn plain_connection(
    ip: IpAddr,
    host: &str,
    port: u16,
) -> Result<HyperStream<TcpStream>> {
    let stream = create_stream(ip, host, port).await?;
    Ok(HyperStream::new_plain(stream))
}
