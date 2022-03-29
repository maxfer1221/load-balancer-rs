use std::sync::{Arc, Mutex};
use tiny_http::Request;
use crate::{backend::Backend, maps, balancer::{Balance, Balancer, ClusterHandle}};

pub fn default() -> Balancer {
    Balancer { cluster_handler: Arc::new(Mutex::new(ClusterHandler::new())) }
}

impl Balance for Balancer {
    fn handle_request(&mut self, mut req: Request) {
        let next_ip = self.cluster_handler.lock().unwrap_or_else(|e| {
            e.into_inner()
        }).get_next_ip();

        let request = match maps::request_map(&mut req, next_ip) {
            Ok(r) => r,
            Err(e) => { println!("{}", e); return },
        };
        println!("{:?}", request);

        let response = request.send();
        // println!("{:?}", response);


        // if(req)

        // req.respond();

        unimplemented!();
    }
}

pub struct ClusterHandler {
    backends: Vec<Backend>,
    current: usize,
}

impl ClusterHandler {
    fn new() -> Self { ClusterHandler { backends: Vec::new(), current: 0 } }
}

impl ClusterHandle for ClusterHandler {
    fn add_backend(&mut self, ip: String) -> Result<(), String> {
        match self.backends.iter().position(|x| (*x).get_ip().eq(&ip)) {
            Some(index) => Err(String::from("DUPLICATE")),
            None => { self.backends.push(Backend::new(ip)); Ok(()) }
        }
    }

    fn remove_backend(&mut self, ip: String) -> Result<(), String> {
        match self.backends.iter().position(|x| (*x).get_ip().eq(&ip)) {
            Some(index) => { self.backends.remove(index); Ok(()) },
            None => Err(String::from("NOT FOUND"))
        }
    }

    fn get_next_ip(&mut self) -> String {
        unimplemented!();
    }

    fn health_check(&self, ip: String) -> Result<bool, String> {
        unimplemented!();
    }
}
