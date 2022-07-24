use std::{thread, collections::HashMap};
use std::sync::{Arc, Mutex, mpsc::{Sender, Receiver}};
use redis;
use crate::{Event, balancer::ClusterHandle};

pub struct Backend;

pub fn create_pubsub(ip: String, tx: Sender<Event>) {
    thread::spawn(move || {

        let mut client = attempt_redis_connection(ip);
        let mut pubsub = client.as_pubsub();
        pubsub.subscribe("backend_add").unwrap();
        pubsub.subscribe("backend_remove").unwrap();

        loop {
            if let Ok(msg) = pubsub.get_message() {
                if let Ok(payload) = msg.get_payload() {
                    println!("{:?}: {}", msg, payload);
                    tx.send(Event::Redis(
                        String::from(msg.get_channel_name()),
                        payload
                    )).unwrap();
                };
            };
        };
    });
}

fn attempt_redis_connection(ip: String) -> redis::Connection {
    let mut redis_connection: Option<redis::Connection> = None;

    while match redis_connection { None => true, _ => false } {
        println!("Attempting to connect to redis cache");

        let redis_client = redis::Client::open(ip.clone()).map_err(|err| {
            println!("Failed to open client: {:?}", err);
            std::thread::sleep(std::time::Duration::from_secs(5));
        }).ok();

        if let Some(c) = redis_client {
            redis_connection = c.get_connection().map_err(|err| {
                println!("Failed to connect: {:?}", err);
                std::thread::sleep(std::time::Duration::from_secs(5));
            }).ok();
        };
    }

    println!("Redis connection established");
    redis_connection.unwrap()
}

pub fn handle_receiver(rx: Receiver<Event>, handler: Arc<Mutex<dyn ClusterHandle>>) {
    loop {
        let event = rx.recv().unwrap_or_else(|err| {
            panic!("Thread channel closed: {:?}.\n Exiting", err)
        });
        match event {
            Event::Redis(method, ip) => {
                match &method[..] {
                    "backend_add" => {
                        let mut guard = handler.lock().unwrap_or_else(|err| { err.into_inner() });
                        (*guard).add_backend(ip).unwrap_or_else(|err| {
                            println!("Error adding backend: {}", err);
                        });
                    },
                    "backend_remove" => {
                        let mut guard = handler.lock().unwrap_or_else(|err| { err.into_inner() });
                        (*guard).remove_backend(ip).unwrap_or_else(|err| {
                            println!("Error removing backend: {}", err);
                        });
                    },
                    _ => {}
                }
                unimplemented!();
            },
            Event::HealthCheck(ip) => {
                unimplemented!();
            }
        }
    }
}
