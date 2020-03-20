/// Common Module
extern crate rand;

use std::collections::HashMap;
use std::collections::VecDeque;
use std::sync::mpsc::SendError;

use rand::Rng;

pub trait LifeCycle {
    fn start(&mut self);
    fn restart(&mut self);
    fn pause(&mut self);
    fn unpause(&mut self);
    fn stop(&mut self);
    fn graceful_stop(&mut self);
}

pub trait Service {
    fn handle(&mut self, op: &String, env: &Envelope);
}

pub trait Producer {
    fn send(&mut self, env: Box<Envelope>);
}

pub trait Consumer {
    fn receive(&mut self) -> Box<Envelope>;
}

pub enum Action{POST, PUT, DELETE, GET}

pub struct Context {

}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Peer {
    pub id: String,
    pub network: Network,
    pub did: DID,
    pub port: u32
}

#[derive(Debug)]
pub struct DID {
    pub username: String,
    pub passphrase: String,
    pub passphrase2: String,
    pub address: String,
    pub algorithm: String
}

#[derive(Debug)]
pub struct Envelope {
    pub id: u64,
    /// A stack-based routing slip that can
    /// be added to at any time prior to
    /// completion.
    pub slip: Slip,
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
    pub payload: Option<String>
}

impl Envelope {
    pub fn new() -> Box<Envelope> {
        let mut rng = rand::thread_rng();
        Box::new(Envelope {
            id: rng.gen(),
            slip: Slip::new(),
            delay_until: 0,
            min_delay: 0,
            max_delay: 0,
            headers: HashMap::new(),
            payload: None
        })
    }
}

#[derive(Debug)]
pub struct Route {
    pub _service: String,
    pub _op: String,
    pub _orig: u64,
    pub _dest: u64,
    pub _from: u64,
    pub _to: u64,
    pub _routed: bool
}

impl Route {
    pub fn new_msg_route_no_relay(orig: u64, dest: u64) -> Route {
        Route {
            _service: String::new(),
            _op: String::new(),
            _orig: orig,
            _dest: dest,
            _from: 0,
            _to: 0,
            _routed: false
        }
    }
    pub fn new_msg_route_with_relay(orig: u64, dest: u64, from: u64, to: u64) -> Route {
        Route {
            _service: String::new(),
            _op: String::new(),
            _orig: orig,
            _dest: dest,
            _from: from,
            _to: to,
            _routed: false
        }
    }
    pub fn new_srv_route_no_relay(service: String, operation: String, orig: u64, dest: u64) -> Route {
        Route {
            _service: service,
            _op: operation,
            _orig: orig,
            _dest: dest,
            _from: 0,
            _to: 0,
            _routed: false
        }
    }
    pub fn new_srv_route_with_relay(service: String, operation: String, orig: u64, dest: u64, from: u64, to: u64) -> Route {
        Route {
            _service: service,
            _op: operation,
            _orig: orig,
            _dest: dest,
            _from: from,
            _to: to,
            _routed: false
        }
    }
}

pub trait Router {
    fn route(&self, env: Box<Envelope>) -> Option<Route>;
}

#[derive(Debug)]
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
    pub fn current_route(&self) -> Option<&Route> {
        self.routes.last()
    }
    pub fn end_route(&mut self) -> Option<Route> {
        self.routes.pop()
    }
    pub fn number_remaining_routes(&self) -> usize {
        self.routes.len()
    }
}

pub mod util {
    pub mod wait {
        use std::{thread, time};
        pub fn wait_a_day(days: u64) {
            thread::sleep(time::Duration::from_secs(days * 24 * 60 * 60));
        }
        pub fn wait_a_hour(hours: u64) {
            thread::sleep(time::Duration::from_secs(hours * 60 * 60));
        }
        pub fn wait_a_minute(minutes: u64) {
            thread::sleep(time::Duration::from_secs(minutes * 60));
        }
        pub fn wait_a_sec(seconds: u64) {
            thread::sleep(time::Duration::from_secs(seconds));
        }
        pub fn wait_a_ms(millis: u64) {
            thread::sleep(time::Duration::from_millis(millis));
        }
        pub fn wait_a_mic(mics: u64) {
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
