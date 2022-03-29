use std::{sync::{mpsc::channel, Arc, Mutex},  io::Cursor, collections::HashMap};
use std::thread;

use tiny_http::{Server, Response, Request, Method, Header};
use clap;

mod sync;
mod balancer;
mod backend;
mod maps;

pub enum Event {
    Redis(String, String),
    HealthCheck(String),
}

fn main() {

    // Initiate Balancer
    let mut balancer = balancer::robin::default();

    // Thread Channel
    let (tx, rx) = channel();

    // Redis Pubsub Checker
    // thread::spawn(move || {
    //     sync::create_pubsub(String::from("redis://redis:6379"), tx.clone());
    // });
    // TODO change back to old hostname
    thread::spawn(move || {
        sync::create_pubsub(String::from("redis://127.0.0.1:6379"), tx.clone());
    });

    // Cluster Handler (manage active IPs)
    thread::spawn(move || {
        let cluster_handler = balancer.get_handler_clone();
        sync::handle_receiver(rx, cluster_handler);
    });
    // try_recv
    // create_pubsub().unwrap();
    // assert_eq!(rx.recv().unwrap(), 10);


    // tiny_http
    let server = Server::http("127.0.0.1:8000").unwrap();

    // Serve requests
    for request in server.incoming_requests() {
        let route = request.url().split('?').collect::<Vec<&str>>().to_vec()[0];

        let response: Response<Cursor<Vec<u8>>> = match route {
            "/" => {
                match request.url().split('=').collect::<Vec<&str>>().to_vec().get(1) {
                    Some(id) => {
                        let mut res = Response::from_string::<String>(
                            format!(
                                "{}\"msg\":\"Hello from server id: {}!\"{}",
                                '{', id, '}'
                            )
                        );
                        res.add_header(Header::from_bytes(
                            &b"Content-Type"[..], &b"application/json"[..]
                        ).unwrap());
                        res
                    },
                    _ => {
                        let mut res = Response::from_string::<String>(String::from("route not found"));
                        res.add_header(Header::from_bytes(
                            &b"Content-Type"[..], &b"text/plain"[..]
                        ).unwrap());
                        res
                    }
                }
            },
            "/ping" => {
                let mut res = Response::from_string::<String>(String::from("pong"));
                res.add_header(Header::from_bytes(
                    &b"Content-Type"[..], &b"text/plain"[..]
                ).unwrap());
                res
            },
            _ => {
                let mut res = Response::from_string::<String>(String::from("route not found"));
                res.add_header(Header::from_bytes(
                    &b"Content-Type"[..], &b"text/plain"[..]
                ).unwrap());
                res
            }
        };

        // respond
        request.respond(response).unwrap();
    }

}

// fn db_handler(_req: &Request) -> Result<Response<File>, Error> {
//     html_response("html/404.html", 404)
//     match req.method() {
//         Method::Post => Response::from_string("POST"),
//         Method::Get => Response::from_string("GET"),
//         _ => Response::from_string("Method not implemented"),
//     }
// }

// fn cache_handler(req: &mut Request, con: &mut redis::Connection, col: &Collection<Var>) -> Result<Response<Cursor<Vec<u8>>>, Error> {
//     let pre_cmd: functions::Function = parse_input(req)?;
//     let cmd: redis::Cmd = pre_cmd.command()?;
//
//     let cache_now = Instant::now();
//     let cache_response: redis::Value = cmd.query(con).map_err(|_e| Error::new(ErrorKind::Other, "Could not apply command to cache"))?;
//     let cache_elapsed = cache_now.elapsed();
//
//     use functions::{Type::*, FunctionType::*};
//
//     let (db_response, db_elapsed) = match pre_cmd.ftype {
//         Set => {
//             let filter: Document = doc! { "name": &pre_cmd.vname };
//             let var: UpdateModifications = UpdateModifications::Document(doc! {
//                 "$set": {
//                     "name": pre_cmd.vname,
//                     "value": match pre_cmd.vtype.unwrap() {
//                         Str(s) => s,
//                         Int(i) => format!("{}", i),
//                     },
//                 }
//             });
//             let update_op = options::FindOneAndUpdateOptions::builder().upsert(true).build();
//             let db_now = Instant::now();
//             (col.find_one_and_update(filter, var, update_op).map_err(|_e| {
//                 println!("{:?}", _e);
//                 Error::new(ErrorKind::Other, "Error finding and updating mongodb document")
//             }),
//             db_now.elapsed())
//         },
//         Get => {
//             let filter: Document = doc! { "name": pre_cmd.vname };
//             let db_now = Instant::now();
//
//             (col.find_one(filter, None).map_err(|_e| {
//                 println!("{:?}", _e);
//                 Error::new(ErrorKind::Other, "Error finding mongodb document")
//             }),
//             db_now.elapsed())
//         },
//         Del => {
//             let filter: Document = doc! { "name": pre_cmd.vname };
//             let db_now = Instant::now();
//             (col.find_one_and_delete(filter, None).map_err(|_e| Error::new(ErrorKind::Other, "Error finding and deleting mongodb document")),
//             db_now.elapsed())
//         },
//     };
//     let r = Response::from_string::<String>(
//         format!("Cache response: {:?}\nTime taken: {:.2?}\nDB response: {:?}\nTime taken: {:.2?}",
//             cache_response,
//             cache_elapsed,
//             db_response,
//             db_elapsed).into());
//     Ok(r)
// }
