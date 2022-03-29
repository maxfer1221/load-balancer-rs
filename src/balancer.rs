use std::sync::{Arc, Mutex};
use tiny_http::Request;

pub mod robin;
pub mod random;

pub struct Balancer {
    cluster_handler: Arc<Mutex<dyn ClusterHandle + Send>>,
}

impl Balancer {
    pub fn get_handler_clone(&mut self) -> Arc<Mutex<dyn ClusterHandle>> {
        self.cluster_handler.clone()
    }
}

pub trait Balance {
    fn handle_request(&mut self, req: Request);
}

pub trait ClusterHandle {
    fn add_backend(&mut self, backend: String) -> Result<(), String>;
    fn remove_backend(&mut self, backend: String) -> Result<(), String>;

    fn get_next_ip(&mut self) -> String;

    fn health_check(&self, ip: String) -> Result<bool, String>;
}
