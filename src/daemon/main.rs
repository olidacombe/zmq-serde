use std::env;
use std::error::Error;
use zeromq::{Socket, SocketRecv};
use zmq_serde::Data;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let mut subscription = "";
    if args.len() > 1 {
        subscription = &args[1];
    }
    let mut socket = zeromq::SubSocket::new();
    socket
        .bind("ipc:///tmp/zmq.exp.sock")
        .await
        .expect("Failed to connect");

    socket.subscribe(subscription).await?;

    loop {
        let recv = socket.recv().await?;
        let r = recv.get(0);
        if r.is_none() {
            continue;
        }
        if let Ok(d) = bincode::deserialize::<Data>(r.unwrap()) {
            dbg!(d);
        }
    }
}
