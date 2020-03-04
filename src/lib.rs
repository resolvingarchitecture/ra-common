
use std::collections::VecDeque;

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
