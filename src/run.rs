// Jackson Coxson
// Stand-alone binary to run EMP

mod lib;

use std::{net::SocketAddrV4, str::FromStr};

use lib::start_loopback;

fn main() {
    simple_logger::init().unwrap();

    let bind_addr_str = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:51820".to_string());

    let bind_addr = SocketAddrV4::from_str(&bind_addr_str)
        .unwrap_or_else(|e| panic!("Invalid bind address '{}': {}", bind_addr_str, e));

    log::info!("Starting EMP on {}", bind_addr);
    let _handle = start_loopback(bind_addr);

    loop {
        std::thread::sleep(std::time::Duration::from_secs(69));
    }
}
