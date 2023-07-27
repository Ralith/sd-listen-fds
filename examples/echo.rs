use std::{io, net::TcpListener, thread};

fn main() {
    let fds = sd_listen_fds::get().unwrap();
    let (_name, fd) = fds
        .into_iter()
        .next()
        .expect("must be launched as a systemd socket-activated service");
    let socket = TcpListener::from(fd);

    loop {
        let (stream, addr) = socket.accept().unwrap();
        thread::spawn(move || {
            println!("{} connected", addr);
            if let Err(e) = io::copy(&mut &stream, &mut &stream) {
                println!("{} connection lost: {}", addr, e);
            }
            println!("{} connection closed", addr);
        });
    }
}
