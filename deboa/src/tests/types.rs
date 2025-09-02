use httpmock::MockServer;

pub const JSONPLACEHOLDER: &str = "https://jsonplaceholder.typicode.com";

pub fn format_address(server: &MockServer) -> String {
    let server_address = *server.address();

    let ip = server_address.ip();
    let port = server_address.port();

    format!("http://{ip}:{port}")
}
