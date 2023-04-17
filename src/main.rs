use std::net::TcpListener;

fn main() {
    let server= TcpListener::bind("127.0.0.1:7001").expect("didn't bind");
    server.set_nonblocking(true).expect("failed in non_blocking");
    let (val, valu)= server.accept().expect("unable to accept");

}
