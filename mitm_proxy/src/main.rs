mod managed_proxy;
mod requests;

use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use managed_proxy::ManagedProxy;
use signal_hook::consts::TERM_SIGNALS;
use signal_hook::flag;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

fn main() {
    let term_now = Arc::new(AtomicBool::new(false));
    for sig in TERM_SIGNALS {
        flag::register_conditional_shutdown(*sig, 1, Arc::clone(&term_now)).ok();
        flag::register(*sig, Arc::clone(&term_now)).ok();
    }

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8100);
    let mut proxy: ManagedProxy = ManagedProxy::new(addr);

    while !term_now.load(Ordering::Relaxed) {
        if let Some(request) = proxy.try_recv_request() {
            request.show_request();
            request.show_response();
        }
    }

    println!("\nReceived kill signal. Wait for cleanup, or hit Ctrl+C again to exit immediately.");
    drop(proxy);
    println!("Cleanup complete. Exiting.");
}
