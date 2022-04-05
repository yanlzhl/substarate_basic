// 引入io、BufReader、Write，{}可一次包含多个 减少代码行数
use std::io::{self, prelude::*, BufReader, Write};
//net模块下的TcpStream 建立tcp连接后的客户端-服务端通道
use std::net::TcpStream;
//字符串切片
use std::str;

fn main() -> std::io::Result<()> {
    //连接绑定本机8080服务
    let mut stream:TcpStream;

    //连接提示
    if let Ok(ok_stream) = TcpStream::connect("127.0.0.1:8080") {
        stream = ok_stream;
        println!("服务端连接成功!");
    } else {
        println!("服务端连接失败，并在此重试一次...");
        stream = TcpStream::connect("127.0.0.1:8080")?;
    }

   

    loop {
        //可变字符变量 用于输入内容的赋值
        let mut input = String::new();

        //获取标准输入 并处理可能发生的错误
        println!("");
        println!("请输入发送内容:");
        io::stdin().read_line(&mut input).expect("错误读取标准输入");
       
        // 将标准输入转为流处理
        stream.write(input.as_bytes()).expect("输入标准输入内容写入流中失败");

        //穿件缓冲读 用于将服务端的流写入缓冲读 提升读取效率
        let mut reader = BufReader::new(&stream);
        
        // 将服务端字节流转入字节队列中
        let mut buffer: Vec<u8> = Vec::new();
        reader.read_until(b'\n', &mut buffer).expect("字节读读取失败");

        //将字节队列中数据转为字符打印
        println!("服务端回复为: {}",  str::from_utf8(&buffer).expect("字节流转为字符失败"));
        println!("");
    }
   
}