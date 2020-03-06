/// Common Module
extern crate rand;

use std::collections::HashMap;
use std::collections::VecDeque;

use rand::Rng;

pub trait LifeCycle {
    fn start(&self);
    fn restart(&self);
    fn pause(&self);
    fn unpaus(&self);
    fn stop(&self);
    fn graceful_stop(&self);
}

pub trait Service {
    fn handle(&self, op: &String, env: &Envelope);
}

pub trait Producer {
    fn send(&self, env: &Envelope);
}

pub trait Consumer {
    fn receive(&self) -> &Envelope;
}

/// Maneuvering Condition
pub enum ManCon {
    NEO,
    EXTREME,
    VERYHIGH,
    HIGH,
    MEDIUM,
    LOW,
    NONE,
    UNKNOWN
}

pub enum Action{POST, PUT, DELETE, GET}

struct Context {

}

impl Context {

}

pub enum Network {
    IMS,
    LiFi,
    Bluetooth,
    WiFiDirect,
    HTTPS,
    TOR,
    I2P,
    Satellite,
    FSRadio
}

pub struct Node {
    pub local_peers: HashMap<Network, Peer>
}

pub struct Peer {
    pub id: String,
    pub network: Network,
    pub did: DID,
    pub port: u32
}

pub struct DID {
    pub username: String,
    pub passphrase: String,
    pub passphrase2: String,

}

pub struct Envelope {
    pub id: u64,
    /// A stack-based routing slip that can
/// be added to at any time prior to
/// completion.
    pub slip: Slip,
    /// The minimal ManCon for this message
    pub man_con: ManCon,
    /// Delay Until this time in milliseconds since epoch.
/// If min_delay and max_delay also included,
/// include a random delay after delay_until based on
/// their range.
    pub delay_until: u64,
    /// Delay for this many milliseconds as a minimum
    pub min_delay: u64,
    /// Delay for this many milliseconds as a maximum
    pub max_delay: u64,
    /// Meta-data used for assisting with routing
    pub headers: HashMap<String, String>,
    /// Data being sent to a destination
    pub payload: HashMap<String, String>
}

impl Envelope {
    fn new() -> Envelope {
        let mut rng = rand::thread_rng();
        Envelope {
            id: rng.gen(),
            slip: Slip::new(),
            man_con: ManCon::UNKNOWN,
            delay_until: 0,
            min_delay: 0,
            max_delay: 0,
            headers: HashMap::new(),
            payload: HashMap::new()
        }
    }
}

pub struct Route {
    pub service: String,
    pub op: String,
    pub orig: String,
    pub orig_port: u16,
    pub dest: String,
    pub dest_port: u16,
    pub from: String,
    pub from_port: u16,
    pub to: String,
    pub to_port: u16,
    pub routed: bool
}

impl Route {
    fn new(s: String, o: String, o_addr: String, o_port: u16, d_addr: String, d_port: u16, f_addr: String, f_port: u16, t_addr: String, t_port: u16) -> Route {
        Route {
            service: s,
            op: o,
            orig: o_addr,
            orig_port: o_port,
            dest: d_addr,
            dest_port: d_port,
            from: f_addr,
            from_port: f_port,
            to: t_addr,
            to_port: t_port,
            routed: false
        }
    }
}

pub struct Slip {
    routes: Vec<Route>,
    in_progress: bool
}

impl Slip {
    fn new() -> Slip {
        Slip {
            routes: Vec::with_capacity(2),
            in_progress: false
        }
    }
    fn with_capacity(capacity: usize) -> Slip {
        Slip {
            routes: Vec::with_capacity(capacity),
            in_progress: false
        }
    }
    pub fn add_route(&mut self, r: Route) {
        self.routes.push(r);
    }
    pub fn next_route(&mut self) -> Option<Route> {
        self.routes.pop()
    }
    pub fn peek_at_next_route(&self) -> Option<&Route> {
        self.routes.last()
    }
    pub fn number_remaining_routes(&self) -> usize {
        self.routes.len()
    }
}

mod util {
    mod wait {
        use std::{thread, time};
        fn wait_a_day(days: u64) {
            thread::sleep(time::Duration::from_secs(days * 24 * 60 * 60));
        }
        fn wait_a_hour(hours: u64) {
            thread::sleep(time::Duration::from_secs(hours * 60 * 60));
        }
        fn wait_a_minute(minutes: u64) {
            thread::sleep(time::Duration::from_secs(minutes * 60));
        }
        fn wait_a_sec(seconds: u64) {
            thread::sleep(time::Duration::from_secs(seconds));
        }
        fn wait_a_ms(millis: u64) {
            thread::sleep(time::Duration::from_millis(millis));
        }
        fn wait_a_mic(mics: u64) {
            thread::sleep(time::Duration::from_micros(mics));
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
