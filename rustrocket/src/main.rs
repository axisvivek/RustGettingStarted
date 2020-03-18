#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use rocket::config::{Config, Environment};





#[get("/hello/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[get("/axismodel/<name>")]
fn axismodel(name: String) -> String {
    format!("Hello, you have requested for Axis camera model : {}!", name)
}

fn main() {
    let config = Config::build(Environment::Staging)
    .address("192.168.1.168")
    .port(8000)
    .finalize().unwrap();

    rocket::custom(config)
    .mount("/", routes![hello])
    .mount("/", routes![axismodel])
    .launch();
     println!("Not able to start.")
    
   
}
