extern crate tokio_proto;

extern crate protocol;

use tokio_proto::TcpServer;

fn main() {
    let addr = "0.0.0.0:666".parse().unwrap();

    let server = TcpServer::new(protocol::Server, addr);

    server.serve(|| Ok(protocol::ServeAddrs));
}
