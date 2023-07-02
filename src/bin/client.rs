use std::{
    io::{self, Read, Write},
    net::TcpStream,
};

use socketry::*;

fn main() -> Null {
    let mut stream = connectsock("127.0.0.1:7777")?;
    let mut inc = "".to_string();
    io::stdin().read_line(&mut inc)?;
    let incb = inc.trim().as_bytes();
    stream.write_all(incb)?;
    stream.flush()?;
    let mut buf = vec![];
    let _u = stream.read_to_end(&mut buf)?;

    println!("{}", &buf.iter().map(|u| *u as char).collect::<String>());
    Ok(())
}

fn connectsock(addr: &str) -> Res<TcpStream> {
    Ok(TcpStream::connect(addr)?)
}
