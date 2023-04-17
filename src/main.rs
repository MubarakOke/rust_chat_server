#[allow(unused_imports)]
use std::net::TcpListener;
#[allow(unused_imports)]
use std::io::{Write, Read, ErrorKind};
#[allow(unused_imports)]
use std::thread::{self, sleep};
#[allow(unused_imports)]
use std::sync::mpsc;
use std::time::Duration;

const LOCAL: &str= "127.0.0.1:7007";

#[allow(unused_variables)]
fn main() {
    println!("setting up server");
    let server= TcpListener::bind(LOCAL).expect("failed to setup server");
    server.set_nonblocking(true).expect("failed to set up non-blocking");
    println!("finished setting up server");

    let mut clients= vec![];
    let (tx, rx)= mpsc::channel::<String>();

    loop {
        if let Ok((mut client, addr)) = server.accept() {
            println!("client connected at {}", addr);
            let info= (client.try_clone().expect("unable to clone"), addr);
            clients.push(info);
            let tx= tx.clone();

            thread::spawn(move || {
                loop {
                    let mut msg= String::new();
                    match client.read_to_string(&mut msg){
                        Ok(_)=>{
                            println!("message {} form {:?}", msg, addr);
                            tx.send(msg).expect("unable to send message");
                        },
                        Err(e) if e.kind() == ErrorKind::WouldBlock => {},
                        Err(e) => {
                            println!("error reading message {}", e);
                            println!("closing connection with {}", addr);
                            break;
                        }
                    }
                    sleep_func();
                };
                }   
            );
            


        }
    }

}

fn sleep_func(){
    sleep(Duration::from_millis(100));
}