use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time;


fn main() -> std::io::Result<()> {
    //监听本地8080端口  0.0.0.0 任意地址均可连接上本端口
    let listener = TcpListener::bind("0.0.0.0:8080")?;

    //判断是否连接正常    https://doc.rust-lang.org/std/net/struct.TcpListener.html
    match listener.accept() {
        //新的客户端连接上时 打印客户端IP
        Ok((_socket, addr)) => println!("新的客户端连接来自: {:?}", addr),
        
        //连接失败
        Err(e) => println!("无法获取客户端信息{:?}", e),
    }
    
    //监听到客户端输入时（流处理）遍历字符内容
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // move 将流的所有权交给子线程处理
                let handle = thread::spawn(move || { handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error));});
                handle.join().unwrap();
            }
            Err(e) => println!("输入流异常{:?}", e),
        }
    }

    //以上无误后 返回OK 结束
    Ok(())
}

/// 对来自客户端的流内容进行处理
fn handle_client(mut stream: TcpStream) -> Result<(), Error>{
    let mut buf = [0; 512];
    loop {
        //根据字节流读取内容 来判断是否有内容
        let bytes_read = stream.read(&mut buf)?;
        //字节为空直接返回
        if bytes_read == 0 {
            return Ok(());
        }

         //打印客户端输入内容
        println!("客户端输入的内容为：{}",String::from_utf8_lossy(&buf[..bytes_read]));

        //客户端输入内容回写
        stream.write(&buf[..bytes_read])?;

        //休眠1秒
        thread::sleep(time::Duration::from_secs(1 as u64));
    }
}