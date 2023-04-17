#[allow(unused_imports)]
use std::net::TcpListener;
#[allow(unused_imports)]
use std::io::{Write, Read, ErrorKind};
#[allow(unused_imports)]
use std::thread::{self, sleep};
#[allow(unused_imports)]
use std::sync::mpsc;

const LOCAL: String= String::from("127.0.0.1:7001");

fn main() {
    println!("setting up server");
    let server= TcpListener::bind(LOCAL).expect("failed to setup server");
    server.set_nonblocking(true).expect("failed to set up non-blocking");
    println!("finished setting up server");

    let mut clients= vec![];
    println!("finished setting up server");

}
