use std::{io::Write, os::unix::net::UnixStream};

const SOCKET: &str = "/tmp/worker.sock";

fn main() -> std::io::Result<()> {
    let mut socket = UnixStream::connect(SOCKET)?;

    let content = String::from("Hello world");

    let bytes = content.as_bytes();
    socket.write_all(bytes)?;

    Ok(())
}
