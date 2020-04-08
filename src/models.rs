/// Common Models Module
use std::collections::HashMap;
use std::convert::{TryFrom};

pub trait Service {
    fn operate(&mut self, operation: u8, env: Envelope);
}

#[derive(Debug, Copy, Clone)]
pub enum ServiceStatus {
    Unregistered        = 0, // Unknown/not registered yet
    // Service Starting Up
    NotInitialized      = 1, // Initial state
    Initializing        = 2, // Initializing service configuration
    Waiting             = 3, // Waiting on a dependent Service status to go to RUNNING
    Starting            = 4, // Starting Service
    Running             = 5, // Service is running normally
    Verified            = 6, // Service has been verified operating normally by receiving a message from it
    PartiallyRunning    = 7, // Service is running normally although not everything is running but it's expected to be normal
    DegradedRunning     = 8, // Service is running but in a degraded manner; likely no need for action, will hopefully come back to Running
    Blocked             = 9, // Service is being blocked from usage
    Unstable            = 10, // Service is running but there could be issues; likely need to restart

    // Service Pausing (Not Yet Supported In Any Service)
    Pausing             = 11, // Service will begin queueing all new requests while in-process requests will be completed
    Paused              = 12, // Service is queueing new requests and pre-pausing requests have completed
    Unpausing           = 13, // Service has stopped queueing new requests and is starting to resume normal operations

    // Service Shutdown
    ShuttingDown            = 14, // Service teardown imminent - not clean, process likely getting killed - perform the minimum ASAP
    GracefullyShuttingDown  = 15, // Ideal clean teardown
    Shutdown                = 16, // Was teardown forcefully - expect potential file / state corruption
    GracefullyShutdown      = 17, // Shutdown was graceful - safe to assume no file / state corruption

    // Restarting
    Restarting          = 18, // Short for GracefullyShuttingDown followed by Initializing on up

    // Unavailable
    Unavailable         = 19, // No Network available but not through blocking, more likely either not installed or not turned on

    // Service Error
    GenError            = 20 // Likely need of Service restart
}

impl ServiceStatus {
    fn as_str(&self) -> &'static str {
        match *self { // *self has type Direction
            ServiceStatus::Unregistered => "Unregistered",
            ServiceStatus::NotInitialized => "NotInitialized",
            ServiceStatus::Initializing => "Initializing",
            ServiceStatus::Waiting => "Waiting",
            ServiceStatus::Starting => "Starting",
            ServiceStatus::Running => "Running",
            ServiceStatus::Verified => "Verified",
            ServiceStatus::PartiallyRunning => "PartiallyRunning",
            ServiceStatus::DegradedRunning => "DegradedRunning",
            ServiceStatus::Blocked => "Blocked",
            ServiceStatus::Unstable => "Unstable",
            ServiceStatus::Pausing => "Pausing",
            ServiceStatus::Paused => "Paused",
            ServiceStatus::Unpausing => "Unpausing",
            ServiceStatus::ShuttingDown => "ShuttingDown",
            ServiceStatus::GracefullyShuttingDown => "GracefullyShuttingDown",
            ServiceStatus::Shutdown => "Shutdown",
            ServiceStatus::GracefullyShutdown => "GracefullyShutdown",
            ServiceStatus::Restarting => "Restarting",
            ServiceStatus::Unavailable => "Unavailable",
            ServiceStatus::GenError => "GenError"
        }
    }
}

impl TryFrom<u8> for ServiceStatus {
    type Error = ();
    fn try_from(original: u8) -> Result<Self, Self::Error> {
        match original {
            0 => Ok(ServiceStatus::Unregistered),
            1 => Ok(ServiceStatus::NotInitialized),
            2 => Ok(ServiceStatus::Initializing),
            3 => Ok(ServiceStatus::Waiting),
            4 => Ok(ServiceStatus::Starting),
            5 => Ok(ServiceStatus::Running),
            6 => Ok(ServiceStatus::Verified),
            7 => Ok(ServiceStatus::PartiallyRunning),
            8 => Ok(ServiceStatus::DegradedRunning),
            9 => Ok(ServiceStatus::Blocked),
            10 => Ok(ServiceStatus::Unstable),
            11 => Ok(ServiceStatus::Pausing),
            12 => Ok(ServiceStatus::Paused),
            13 => Ok(ServiceStatus::Unpausing),
            14 => Ok(ServiceStatus::ShuttingDown),
            15 => Ok(ServiceStatus::GracefullyShuttingDown),
            16 => Ok(ServiceStatus::Shutdown),
            17 => Ok(ServiceStatus::GracefullyShutdown),
            18 => Ok(ServiceStatus::Restarting),
            19 => Ok(ServiceStatus::Unavailable),
            20 => Ok(ServiceStatus::GenError),
            n => Err(())
        }
    }
}

pub trait Producer {
    fn send(&mut self, env: Envelope);
}

pub trait Consumer {
    fn receive(&mut self) -> Envelope;
}

#[derive(Debug, Copy, Clone)]
pub enum Action {
    POST    = 0,
    PUT     = 1,
    DELETE  = 2,
    GET     = 3
}

#[derive(Debug, Copy, Clone)]
pub enum NetworkId {
    IMS        = 0,
    LiFi       = 1,
    Bluetooth  = 2,
    WiFiDirect = 3,
    HTTPS      = 4,
    VPN        = 5,
    TOR        = 6,
    I2P        = 7,
    Satellite  = 8,
    FSRadio    = 9
}

impl TryFrom<u8> for NetworkId {
    type Error = ();
    fn try_from(original: u8) -> Result<Self, Self::Error> {
        match original {
            0 => Ok(NetworkId::IMS),
            1 => Ok(NetworkId::LiFi),
            2 => Ok(NetworkId::Bluetooth),
            3 => Ok(NetworkId::WiFiDirect),
            4 => Ok(NetworkId::HTTPS),
            5 => Ok(NetworkId::VPN),
            6 => Ok(NetworkId::TOR),
            7 => Ok(NetworkId::I2P),
            8 => Ok(NetworkId::Satellite),
            9 => Ok(NetworkId::FSRadio),
            _ => Err(())
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum NetworkStatus {
    Unregistered           = 0, // Unknown/not registered yet

    // Network Client Starting Up
    NotInitialized         = 1, // Initial state - Registered
    Initializing           = 2, // Initializing Network Client's environment including configuration of Networking component
    Starting               = 3, // Starting of Networking component
    Waiting                = 4,  // Means this Network Client is waiting on a dependent Network Client's status to change to Starting, e.g. Bote waiting on I2P to begin starting up.
    // Network Networking
    NetworkWarmup          = 5, // Means this Network Client is waiting for a dependent Network Client's status to change to NetworkConnected, e.g. Bote waiting on I2P to actually connect.
    NetworkPortConflict    = 6, // Means this Network Client was unable to open the supplied port - likely being blocked; recommend changing ports
    NetworkConnecting      = 7, // Attempting to connect with network
    NetworkConnected       = 8, // Network successfully connected and ready to handle requests
    NetworkVerified        = 9, // Network has claimed to be connected (NetworkConnected) and we have received a message from the network verifying it is
    NetworkStopping        = 10, // Network connection is hanging, e.g. unacceptable response times, begin looking at alternatives
    NetworkStopped         = 11, // Network connection failed, try another or recommend alternative
    NetworkBlocked         = 12, // Network connection being blocked.
    NetworkUnavailable     = 13, // Network is not available; either not installed in machine or not started
    NetworkError           = 14, // Error in Network; handle within Network Client if possible yet make Network Client Service aware of likely service degradation.
    // Network Client Pausing (Not Yet Supported In Any Network)
    Pausing                = 15, // Queueing up requests both inbound and outbound waiting for pre-pausing requests to complete.
    Paused                 = 16, // All pre-pausing requests completed.
    Unpausing              = 17, // Unblocking queued requests to allow them to continue on and not queueing further requests.
    // Network Client Shutdown
    ShuttingDown           = 18, // Shutdown imminent - not clean, process likely getting killed - perform the minimum ASAP
    GracefullyShuttingDown = 19, // Ideal clean teardown
    Shutdown               = 20, // Was teardown forcefully - expect potential file / state corruption
    GracefullyShutdown     = 21, // Shutdown was graceful - safe to assume no file / state corruption
    // Network Client Restarting
    Restarting             = 22, // Short for GracefullyShuttingDown then STARTING back up.
    // Network Client Error
    GenError               = 23 // Likely need of Network Client restart
}

impl TryFrom<u8> for NetworkStatus {
    type Error = ();
    fn try_from(original: u8) -> Result<Self, Self::Error> {
        match original {
            0 => Ok(NetworkStatus::Unregistered),
            1 => Ok(NetworkStatus::NotInitialized),
            2 => Ok(NetworkStatus::Initializing),
            3 => Ok(NetworkStatus::Starting),
            4 => Ok(NetworkStatus::Waiting),
            5 => Ok(NetworkStatus::NetworkWarmup),
            6 => Ok(NetworkStatus::NetworkPortConflict),
            7 => Ok(NetworkStatus::NetworkConnecting),
            8 => Ok(NetworkStatus::NetworkConnected),
            9 => Ok(NetworkStatus::NetworkVerified),
            9 => Ok(NetworkStatus::NetworkStopping),
            9 => Ok(NetworkStatus::NetworkStopped),
            9 => Ok(NetworkStatus::NetworkBlocked),
            9 => Ok(NetworkStatus::NetworkUnavailable),
            9 => Ok(NetworkStatus::NetworkError),
            9 => Ok(NetworkStatus::Pausing),
            9 => Ok(NetworkStatus::Paused),
            9 => Ok(NetworkStatus::Unpausing),
            9 => Ok(NetworkStatus::ShuttingDown),
            9 => Ok(NetworkStatus::GracefullyShuttingDown),
            9 => Ok(NetworkStatus::Shutdown),
            9 => Ok(NetworkStatus::GracefullyShutdown),
            9 => Ok(NetworkStatus::Restarting),
            9 => Ok(NetworkStatus::GenError),
            _ => Err(())
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum PacketType {
    Data  = 0, // packet carries a data payload
    Fin   = 1, // signals the end of a connection
    Ack   = 2, // signals acknowledgment of a packet
    Reset = 3, // forcibly terminates a connection
    Syn   = 4, // initiates a new connection with a peer
}

impl TryFrom<u8> for PacketType {
    type Error = ();
    fn try_from(original: u8) -> Result<Self, Self::Error> {
        match original {
            0 => Ok(PacketType::Data),
            1 => Ok(PacketType::Fin),
            2 => Ok(PacketType::Ack),
            3 => Ok(PacketType::Reset),
            4 => Ok(PacketType::Syn),
            n => Err(())
        }
    }
}

#[derive(Debug, Clone)]
// #[derive(Serialize,Deserialize)]
pub struct Packet {
    pub packet_type: u8,
    // Temporary identification of this packet between from and to address.
    // Normally used for Claim Checks and/or to ensure packet was received then discarded.
    pub id: u8,
    // Network this packet was sent over
    pub network_id: u8,
    // Sender node's address
    pub from_addr: String,
    // Destination node's address
    pub to_addr: String,
    // Delay for this many seconds as a minimum
    pub min_delay: u16,
    // Delay for this many seconds as a maximum
    pub max_delay: u16,
    // Data being sent
    pub envelope: Option<Envelope>
}

impl Packet {
    pub fn new(id: u8, packet_type: u8, network_id: u8, from_addr: String, to_addr: String, envelope: Option<Envelope>) -> Packet {
        Packet { id, packet_type, network_id, from_addr, to_addr, min_delay: 0, max_delay: 0, envelope }
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

#[derive(Debug, Clone)]
/// An Envelope is a wrapper of data with some meta-data for internal routing.
pub struct Envelope {
    pub from: u8,
    pub to: u8,
    pub msg: Vec<u8>,
    /// A stack-based routing slip that can
    /// be added to at any time prior to
    /// completion.
    pub slip: Slip
}

impl Envelope {
    pub fn new(from: u8, to: u8, msg: Vec<u8>) -> Envelope {
        Envelope { from, to, msg, slip: Slip::new()}
    }
    pub fn new_msg_only(msg: Vec<u8>) -> Envelope {
        Envelope {from: 0, to: 0, msg, slip: Slip::new()}
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Route {
    pub service: u8,
    pub op: u8,
}

impl Route {
    pub fn new(service: u8, op: u8) -> Route {
        Route { service, op }
    }
}

/// Provides a vector of Route implemented as a Stack.
/// Supports adding to the stack at any point.
#[derive(Debug, Clone)]
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