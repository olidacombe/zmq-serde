use rand::Rng;
use std::time::Duration;
use zeromq::*;
use zmq_serde::Data;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let mut rng = rand::thread_rng();
    // let stocks: Vec<&str> = vec!["AAA", "ABB", "BBB"];
    println!("Starting pub");
    let mut socket = zeromq::PubSocket::new();
    socket.connect("ipc:///tmp/zmq.exp.sock").await?;

    println!("Start sending loop");
    loop {
        let d = Data::rand();
        let s = bincode::serialize(&d)?;
        let m = ZmqMessage::from(s);
        socket.send(m).await?;
        tokio::time::sleep(Duration::from_secs(1)).await
    }
}
