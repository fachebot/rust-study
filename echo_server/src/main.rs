use std::env;
use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_connection(addr: &str, mut stream: TcpStream) -> Result<(), Error> {
    let mut buf = [0; 1024];

    loop {
        // 读取对端数据
        let size = stream.read(&mut buf)?;

        // 是否断开连接
        if size == 0 {
            return Ok(());
        }

        // 输出接收消息
        let message = String::from_utf8_lossy(&buf[0..size]);
        println!("[{}] recv: {:?}", addr, message);

        // 写入读取到的数据
        stream.write_all(&buf[0..size])?;
        stream.flush()?;
        println!("[{}] send: {:?}", addr, message);
    }
}

fn main() {
    // 解析端口参数
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: echo_server [port]");
        return;
    }
    let port: u16 = args[1].parse::<u16>().unwrap();

    // 监听指定端口
    let addr = format!("0.0.0.0:{0}", port);
    let listener = TcpListener::bind(addr).unwrap();

    // 接受新的连接
    for stream in listener.incoming() {
        // 处理接受连接错误
        if let Err(err) = stream {
            println!("failed to accept connection, {:?}", err);
            continue;
        }

        // 处理获取地址错误
        let stream = stream.unwrap();
        let addr = stream.peer_addr();
        if let Err(err) = addr {
            println!("failed to get peer addr, {:?}", err);
            continue;
        }

        // 输出对端地址信息
        let addr = addr.unwrap().to_string();
        println!("[{}] accepted", addr);

        // 创建新线程处理连接
        thread::spawn(move || match handle_connection(addr.as_str(), stream) {
            Ok(()) => println!("[{}] disconnected", addr),
            Err(e) => println!("[{}] connection error: {:?}", addr, e),
        });
    }
}
