use std::{io::Write, thread::sleep, time::Duration};
use wasmedge_wasi_socket::{Shutdown, TcpStream};

fn main() -> std::io::Result<()> {
    let port = std::env::var("PORT").unwrap_or("1234".to_string());
    println!("connect to 127.0.0.1:{}", port);
    let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port))?;
    println!("local address {}", stream.local_addr().unwrap());
    println!("peer address {}", stream.peer_addr().unwrap());
    println!("sending hello message...");
    let mut i = 0;
    while i< 100 {

        stream.write(b"hello")?;

        sleep(Duration::from_millis(500));

        i=i+1;
    }
    stream.shutdown(Shutdown::Both)?;
    Ok(())
}