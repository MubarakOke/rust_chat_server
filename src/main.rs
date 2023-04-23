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
#[allow(unused_mut)]
fn main() {
    println!("setting up server");
    let server= TcpListener::bind(LOCAL).expect("failed to setup server");
    server.set_nonblocking(true).expect("failed to set up non-blocking");
    println!("finished setting up server");

    let mut clients= vec![];
    let (tx, rx)= mpsc::channel::<String>();

    println!("waiting fot incoming connection.............");

    loop {
        if let Ok((mut client, addr)) = server.accept() {
            println!("client connected at {}", addr);
            let info= (client.try_clone().expect("unable to clone client"), addr);
            clients.push(info);
            let tx= tx.clone();

            thread::spawn(move || {
                loop {
                    let mut msg= String::new();
                    // let mut msg= msg.as_bytes();
                    // client.read_to_string(&mut msg).expect("error");
                    // let mut buffer = [0; 1];
                    let mut buffer = Vec::new();
                    match client.read_to_end(&mut buffer){
                        Ok(f)=>{
                            // println!("message {} form {}", msg.to_string(), addr);
                            // tx.send(msg.to_str).expect("unable to send message");
                            println!("{}",f);
                        },
                        Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                            println!("empty");
                        },
                        Err(e) => {
                            println!("closing connection with {}", addr);
                            break;
                        }
                    }
                    println!("after match");
                    sleep_func();
                };
                }   
            );
        }

        if let Ok(msg)= rx.try_recv() {
            clients= clients.into_iter().map(|mut item|{
                if item.0.local_addr().expect("unable to get local address") == item.1 {
                    return item;
                }
                item.0.write_all(msg.as_bytes()).expect("un able to send message");
                item
            }).collect();
        }

        sleep_func()
    }

}

fn sleep_func(){
    sleep(Duration::from_millis(2500));
}