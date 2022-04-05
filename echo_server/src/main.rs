use async_std::io::{ReadExt, WriteExt};
use async_std::net::{TcpListener, TcpStream};
use async_std::task;
use std::env;

async fn handle_connection(addr: &str, mut socket: TcpStream) -> Result<(), std::io::Error> {
    let mut buf = [0; 1024];

    loop {
        // 读取对端数据
        let size = socket.read(&mut buf).await?;

        // 是否关闭连接
        if size == 0 {
            return Ok(());
        }

        // 输出接收消息
        let message = String::from_utf8_lossy(&buf[0..size]);
        println!("[{}] recv: {:?}", addr, message);

        // 写入读取到的数据
        socket.write_all(&buf[0..size]).await?;
        socket.flush().await?;
        println!("[{}] send: {:?}", addr, message);
    }
}

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    // 解析端口参数
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: echo_server [port]");
        return Ok(());
    }
    let port: u16 = args[1].parse::<u16>().unwrap();

    // 监听指定端口
    let addr = format!("0.0.0.0:{0}", port);
    let listener = TcpListener::bind(addr.as_str()).await?;

    // 轮询接受连接并处理
    loop {
        // 接受新的连接
        let (socket, peer_addr) = match listener.accept().await {
            Err(err) => {
                eprintln!("failed to accept connection, {}", err);
                continue;
            }
            Ok((socket, peer_addr)) => (socket, peer_addr.to_string()),
        };

        println!("[{}] accepted", peer_addr);

        task::spawn(async move {
            match handle_connection(peer_addr.as_str(), socket).await {
                Ok(()) => println!("[{}] disconnected", peer_addr),
                Err(err) => eprintln!("[{}] connection error: {:?}", peer_addr, err),
            }
        });
    }
}
