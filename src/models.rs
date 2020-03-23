/// Common Models Module
extern crate rand;

use std::collections::HashMap;
use std::collections::VecDeque;
use std::sync::mpsc::SendError;
use std::marker::Send;

use rand::Rng;

pub trait Service {
    fn handle(&mut self, op: &String, env: &mut Envelope);
}

pub enum ServiceStatus {
    // Service Starting Up
    NotInitialized, // Initial state
    Initializing, // Initializing service configuration
    Waiting, // Waiting on a dependent Service status to go to RUNNING
    Starting, // Starting Service
    Running, // Service is running normally
    Verified, // Service has been verified operating normally by receiving a message from it
    PartiallyRunning, // Service is running normally although not everything is running but it's expected to be normal
    DegradedRunning, // Service is running but in a degraded manner; likely no need for action, will hopefully come back to Running
    Blocked, // Service is being blocked from usage
    Unstable, // Service is running but there could be issues; likely need to restart
    // Service Pausing (Not Yet Supported In Any Service)
    Pausing, // Service will begin queueing all new requests while in-process requests will be completed
    Paused, // Service is queueing new requests and pre-pausing requests have completed
    Unpausing, // Service has stopped queueing new requests and is starting to resume normal operations
    // Service Shutdown
    ShuttingDown, // Service teardown imminent - not clean, process likely getting killed - perform the minimum ASAP
    GracefullyShuttingDown, // Ideal clean teardown
    Shutdown, // Was teardown forcefully - expect potential file / state corruption
    GracefullyShutdown, // Shutdown was graceful - safe to assume no file / state corruption
    // Restarting
    Restarting, // Short for GracefullyShuttingDown followed by Initializing on up
    // Unavailable
    Unavilable, // No Network available but not through blocking, more likely either not installed or not turned on
    // Service Error
    Error // Likely need of Service restart
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
pub enum NetworkId {
    IMS,
    LiFi,
    Bluetooth,
    WiFiDirect,
    HTTPS,
    VPN,
    TOR,
    I2P,
    Satellite,
    FSRadio
}

#[derive(Debug)]
pub enum NetworkStatus {
    Unregistered, // 0 - Unknown/not registered yet
    // Sensor Starting Up
    NotInitialized, // 1 - Initial state - Registered
    Initializing, // 2 - Initializing Sensor's environment including configuration of Networking component
    Starting, // 3 - Starting of Networking component
    Waiting,  // Optional 3.1 - means this sensor is waiting on a dependent sensor's status to change to Starting, e.g. Bote waiting on I2P to begin starting up.
    // Sensor Networking
    NetworkWarmup, // Optional 3.2 - means this sensor is waiting for a dependent sensor's status to change to NetworkConnected, e.g. Bote waiting on I2P to actually connect.
    NetworkPortConflict, // Optional 3.3 - means this sensor was unable to open the supplied port - likely being blocked; recommend changing ports
    NetworkConnecting, // 4 - Attempting to connect with network
    NetworkConnected, // 5 - Network successfully connected and ready to handle requests
    NetworkVerified, // 6 - Network has claimed to be connected (NetworkConnected) and we have received a message from the network verifying it is
    NetworkStopping, // Network connection is hanging, e.g. unacceptable response times, begin looking at alternatives
    NetworkStopped, // Network connection failed, try another or recommend alternative
    NetworkBlocked, // Network connection being blocked.
    NetworkUnavailable, // Network is not available; either not installed in machine or not started
    NetworkError, // Error in Network; handle within Sensor if possible yet make Sensor Service aware of likely service degradation.
    // Sensor Pausing (Not Yet Supported In Any Sensors)
    Pausing, // Queueing up requests both inbound and outbound waiting for pre-pausing requests to complete.
    Paused, // All pre-pausing requests completed.
    Unpausing, // Unblocking queued requests to allow them to continue on and not queueing further requests.
    // Sensor Shutdown
    ShuttingDown, // Shutdown imminent - not clean, process likely getting killed - perform the minimum ASAP
    GracefullyShuttingDown, // Ideal clean teardown
    Shutdown, // Was teardown forcefully - expect potential file / state corruption
    GracefullyShutdown, // Shutdown was graceful - safe to assume no file / state corruption
    // Sensor Restarting
    Restarting, // Short for GracefullyShuttingDown then STARTING back up.
    // Sensor Error
    Error // Likely need of Sensor restart
}

#[derive(Debug)]
pub struct Network {
    _id: NetworkId,
    _status: NetworkStatus
}

impl Network {
    pub fn new(id: NetworkId) -> Box<Network> {
        Box::new(Network {
            _id: id,
            _status: NetworkStatus::NotInitialized
        })
    }
}

pub struct Node {
    pub local_peers: HashMap<NetworkId, Peer>
}

#[derive(Debug)]
pub struct Peer {
    pub network_id: NetworkId,
    pub did: DID
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
pub struct Packet {
    pub network: u16,
    pub from_addr: [u8],
    pub to_addr: [u8],
    pub sig: [u8],
    pub payload: [u8]
}

// impl Packet {
//     pub fn serialize(&mut packet: Packet) -> [u8] {
//
//     }
//
//     pub fn deserialize(packet: [u8]) -> Packet {
//
//     }
// }

#[derive(Debug)]
pub struct Envelope {
    pub id: u64,
    /// Optional signature of originator for authentication
    pub sig: Option<String>,
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
    pub payload: HashMap<String, String>
}

unsafe impl Send for Envelope {}

impl Envelope {
    pub fn new() -> Box<Envelope> {
        let mut rng = rand::thread_rng();
        Box::new(Envelope {
            id: rng.gen(),
            sig: None,
            slip: Slip::new(),
            delay_until: 0,
            min_delay: 0,
            max_delay: 0,
            headers: HashMap::new(),
            payload: HashMap::new()
        })
    }
}

#[derive(Debug)]
pub struct Route {
    pub _service: String,
    pub _op: String,
}

impl Route {
    pub fn new(service: String, operation: String) -> Route {
        Route {
            _service: service,
            _op: operation,
        }
    }
}

/// Provides a vector of Route implemented as a Stack.
/// Supports adding to the stack at any point.
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}