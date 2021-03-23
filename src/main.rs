use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() {
    // 在本机监听7878端口
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // 打印监听的端口
    println!("Server listening on port 7878");
    // 依次处理来自客户端的连接
    for stream in listener.incoming() {
        // 以模式匹配的方式来处理连接
        match stream {
            Ok(s) => {
                // 打印提示信息
                println!("New client!");
                // 调用处理连接的函数
                handle_connection(s);
            }
            Err(e) => {
                // 打印错误信息
                panic!("Error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    // 声明一个用于存放数据的缓冲区
    let mut buf = [0; 512];
    loop {
        // 从 TcpStream 中读取数据并将其存储在缓冲区中
        let bytes_read = stream.read(&mut buf).unwrap();
        // 将缓冲区中的字节转换成字符串并打印出来
        println!("Request: {}", String::from_utf8_lossy(&buf[..bytes_read]));
        // 将缓冲区中的数据echo返回给客户端
        stream.write(&buf[..bytes_read]).unwrap();
    }
}