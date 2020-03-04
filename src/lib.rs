use std::collections::HashMap;
use std::collections::VecDeque;

pub enum ManCon {
    NEO, EXTREME, VERYHIGH, HIGH, MEDIUM, LOW, NONE, UNKNOWN
}

struct Context {

}

impl Context {

}

pub struct Envelope {
    pub id: i64,
    pub headers: HashMap<String, String>,
    pub client: i64,
    pub reply_to_client: bool,
    pub client_reply_action: String,
    pub external: bool,
    pub routing_slip: DynamicRoutingSlip,
    pub man_con: ManCon,
    pub min_delay: i64,
    pub max_delay: i64,
    pub payload: HashMap<String, String>
}

impl Envelope {

}

pub struct Route {
    pub id: i64,
    pub service: String,
    pub op: String,
    pub orig: String,
    pub dest: String,
    pub to: String,
    pub from: String,
    pub routed: bool
}

pub struct DynamicRoutingSlip {
    routes: VecDeque<Route>,
    in_progress: bool
}

impl DynamicRoutingSlip {
    pub fn start(&self) {}
    pub fn next_route(&mut self) {}
    pub fn peek_at_next_route(&self) {}
    pub fn number_remaining_routes(&self) {}
}

trait LifeCycle {
    fn start(&self) -> bool;
    fn restart(&self) -> bool;
    fn pause(&self) -> bool;
    fn unpaus(&self) -> bool;
    fn stop(&self) -> bool;
    fn graceful_stop(&self) -> bool;
}

trait Service {
    fn handle(&self);
}

trait MessageProducer {

}

trait MessageConsumer {

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
