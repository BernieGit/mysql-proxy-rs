//! MySQL Proxy Server
extern crate mysql_proxy;

use mysql_proxy::*;

fn main() {

    env_logger::init().unwrap();
    let addr = env::args().nth(1).unwrap_or("127.0.0.1:3307".to_string());
    let addr = addr.parse::<SocketAddr>().unwrap();

    let proxy = Proxy { bind: addr };

    proxy.run();
}

/// A simple handler that just prints packets and then allows them to pass through
struct PassthroughHandler {}

impl PacketHandler for PassthroughHandler {

    fn handle_request(&self, p: &Packet) -> Action {
        print_packet_chars(&p);
        Action::Forward
    }

    fn handle_response(&self, p: &Packet) -> Action {
        print_packet_chars(&p);
        Action::Forward
    }

}
#[allow(dead_code)]
pub fn print_packet_chars(buf: &[u8]) {
    print!("[");
    for i in 0..buf.len() {
        print!("{} ", buf[i] as char);
    }
    println!("]");
}

#[allow(dead_code)]
pub fn print_packet_bytes(buf: &[u8]) {
    print!("[");
    for i in 0..buf.len() {
        if i%8==0 { println!(""); }
        print!("{:#04x} ",buf[i]);
    }
    println!("]");
}


