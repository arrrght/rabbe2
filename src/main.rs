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

use clap::{App, SubCommand};

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
    let matches = App::new("rabbe2")
        .subcommand(SubCommand::with_name("publisher").about("publish messages"))
        .subcommand(
            SubCommand::with_name("consumer")
                .about("consume")
                .arg_from_usage("-t, --timeout=[timeout] 'Heartbeat timeout'"),
        )
        .get_matches();

    if let ("consumer", Some(cmd)) = matches.subcommand() {
        consumer::run(&cmd);
    } else {
        println!("run as dnsdg ping");
    }

    match matches.subcommand_name() {
        Some("both") => println!("method 'both' is'n written yet"),
        Some("publisher") => {
            println!("running publisher");
            publisher::run();
        }
        None => println!("use --help"),
        _ => println!("use --help"),
    }
}
