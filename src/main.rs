#[macro_use] extern crate log;
extern crate env_logger;

#[macro_use]
extern crate tower_web;

use tower_web::ServiceBuilder;

extern crate poc_rust_tower;

use crate::poc_rust_tower::apis;

pub fn main() {
    let _ = env_logger::try_init();

    let addr = "127.0.0.1:8080".parse().expect("Invalid address");
    println!("Listening on http://{}", addr);

    ServiceBuilder::new()
        .resource(apis::table::Endpoints)
        .run(&addr)
        .unwrap();
}
