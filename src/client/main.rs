use rand::Rng;
use std::time::Duration;
use zeromq::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();
    let stocks: Vec<&str> = vec!["AAA", "ABB", "BBB"];
    println!("Starting pub");
    let mut socket = zeromq::PubSocket::new();
    socket.connect("ipc:///tmp/zmq.exp.sock").await?;

    println!("Start sending loop");
    loop {
        for stock in &stocks {
            let price: u32 = rng.gen_range(1..100);
            let mut m: ZmqMessage = ZmqMessage::from(*stock);
            m.push_back(price.to_ne_bytes().to_vec().into());
            dbg!(m.clone());
            socket.send(m).await?;
        }
        tokio::time::sleep(Duration::from_secs(1)).await
    }
}
