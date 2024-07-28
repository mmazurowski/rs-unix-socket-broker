use std::borrow::BorrowMut;
use std::fs;
use std::io::prelude::*;
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::Path;
mod pool;

use env_logger::Env;

use log::info;

const SOCKET: &str = "/tmp/worker.sock";

struct Socker {
    listener: UnixListener,
}

impl Socker {
    pub fn new() -> std::io::Result<Socker> {
        let path = Path::new(SOCKET);

        if path.exists() {
            fs::remove_file(SOCKET).expect("Must exist");
        }

        let sock = UnixListener::bind(SOCKET)?;

        Ok(Socker { listener: sock })
    }
}

fn handler(mut stream: UnixStream) {
    let copied = stream.borrow_mut();
    let mut message = String::new();

    copied.read_to_string(&mut message).unwrap();

    info!("Received message, {}", message);
}

fn main() -> std::io::Result<()> {
    let env = Env::default().filter("RS_WORKER");
    env_logger::init_from_env(env);

    let socker = Socker::new()?;
    let thread_pool = pool::ThreadPool::new(10);

    info!("Starting listening on socket {}", SOCKET);

    for stream in socker.listener.incoming() {
        let valid_stream = match stream {
            Ok(val) => val,
            Err(err) => {
                eprint!("Failed reading on socket: {}", err);
                break;
            }
        };

        thread_pool.execute(|| handler(valid_stream));
    }

    Ok(())
}
