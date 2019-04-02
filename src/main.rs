use lapin_futures as lapin;

use crate::lapin::channel::{BasicProperties, BasicPublishOptions, QueueDeclareOptions};
use clap::value_t;
use clap::App;
mod consumer;
mod publisher;

const RBT_USER: &str = "guest";
const RBT_PASSWORD: &str = "guest";
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

#[derive(Debug, Clone)]
pub struct Opt {
    timeout: u16,
    count_messages: u32,
    sleep: u64,
    queue: String,
    save: bool,
    queue_options: QueueDeclareOptions,
    dcount: usize,
}

fn digicount(a: u32) -> usize {
    ((a as f64).log10() + 1.0) as usize
}

fn main() {
    env_logger::init();
    let app = App::new("rabbe2")
        .arg_from_usage("-c, --consumer 'run consumer'")
        .arg_from_usage("-p, --publisher 'run publisher'")
        .arg_from_usage("-a, --add 'add some messages to queue'")
        .arg_from_usage("-s, --save-file 'Save messages to file'")
        .arg_from_usage("-q, --queue[some] 'rabbit's queue name'")
        .arg_from_usage("-t, --timeout[sec] 'Heartbeat timeout'")
        .arg_from_usage("-T, --sleep[msec] 'Sleep between publish'")
        .arg_from_usage("-C, --count[9999] 'Process n messages'")
        .arg_from_usage("-r, --read 'Read messages from dir'");
    let matches = app.clone().get_matches();

    let prm = Opt {
        timeout: value_t!(matches, "timeout", u16).unwrap_or(5),
        count_messages: value_t!(matches, "count", u32).unwrap_or(9999),
        dcount: digicount(value_t!(matches, "count", u32).unwrap_or(9999)),
        sleep: value_t!(matches, "sleep", u64).unwrap_or(500),
        queue: value_t!(matches, "queue", String).unwrap_or("some".to_string()),
        save: match matches.is_present("save-file") {
            true => true,
            _ => false,
        },
        queue_options: QueueDeclareOptions {
            //durable: true,
            ..Default::default()
        },
    };
    //println!("PRM:{:?} ::: {:?}", prm, matches);
    //std::process::exit(0);

    let mut children = vec![];

    if matches.is_present("consumer") {
        println!("spawn consumer");
        let matches = matches.clone();
        let prm2 = prm.clone();
        children.push(std::thread::spawn(move || {
            consumer::run(&matches, prm2);
        }));
    };

    if matches.is_present("publisher") {
        println!("spawn publisher");
        let matches = matches.clone();
        children.push(std::thread::spawn(move || {
            publisher::run(&matches, prm.clone());
        }));
    };

    for child in children {
        let _ = child.join();
    }
}
