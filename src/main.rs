use lapin_futures as lapin;
//use crate::lapin::channel::{BasicProperties, BasicPublishOptions, QueueDeclareOptions};
//use crate::lapin::client::ConnectionOptions;
//use crate::lapin::types::FieldTable;
//use env_logger;
//use failure::Error;
//use futures::future::Future;

//use tokio;
//use tokio::net::TcpStream;
//use tokio::runtime::Runtime;

//use log::info;
//use std::net::SocketAddr;
//use std::io::{self, Write};

use clap::App;

mod consumer;
mod publisher;

//const RBT_QUEUE: &str = "pikapika";
//const RBT_USER: &str = "crawler1";
//const RBT_PASSWORD: &str = "crawler1";
pub const SLEEP_MILLIS: u64 = 500;
const RBT_USER: &str = "guest";
const RBT_PASSWORD: &str = "guest";
const RBT_QUEUE: &str = "some";
const RBT_MESSAGE: &str = r#"{
  "head": {
    "request": {
      "special": "x",
      "uid": "test_short_for_nic_eyJ0eXBlIjoiR1JaIiwiYm9keSI6ItCwMTEx0LDQsDc3NyJ9_a6605a8e6f2c406b9c9e0a0a5fbe3538@test4",
      "report_uid": "test_short_for_nic_eyJ0eXBlIjoiR1JaIiwiYm9keSI6ItCQMTEx0JDQkDc3NyJ9@test4",
      "stamp": 1550669996839,
      "type": "DATA",
      "version": "1.0.0.1",
      "query": {
        "type": "GRZ",
        "body": "А111АА777"
      },
      "report_type_uid": "test_short_for_nic@test4",
      "domain_uid": "test4",
      "user_uid": "nic_test@test4",
      "agent_uid": "arapi-default_agent",
      "fields": [],
      "sources": [
        "base"
      ]
    }
  }
}"#;

fn main() {
    env_logger::init();
    let mut app = App::new("rabbe2")
        .arg_from_usage("-t, --timeout=[timeout] 'Heartbeat timeout'")
        .arg_from_usage("-c, --consumer 'run consumer'")
        .arg_from_usage("-p, --publisher 'run publisher'");
    let matches = app.clone().get_matches();

    let mut children = vec![];
    if matches.is_present("consumer") {
        println!("spawn consumer");
        let matches = matches.clone();
        children.push(std::thread::spawn(move || {
            consumer::run(&matches);
        }));
    };

    if matches.is_present("publisher") {
        println!("spawn publisher");
        let matches = matches.clone();
        children.push(std::thread::spawn(move || {
            publisher::run(&matches);
        }));
    };

    if !matches.is_present("publisher") &&  !matches.is_present("consumer")  {
        app.print_help().unwrap();
        println!("");
    }

    for child in children {
        let _ = child.join();
    }

}
