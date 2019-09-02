use lapin_futures as lapin;

use crate::lapin::channel::QueueDeclareOptions;
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
    parall: usize,
    is_read: bool,
    is_add: bool,
}

fn digicount(a: u32) -> usize {
    ((a as f64).log10() + 1.0) as usize
}

fn main() {
    env_logger::init();
    let app = App::new("rabbe2")
        .arg_from_usage("-c, --consumer 'Run consumer'")
        .arg_from_usage("-p, --publisher 'Run publisher'")
        .arg_from_usage("-a, --add 'Add some messages to queue'")
        .arg_from_usage("-s, --save-file 'Save messages to file into subdir messages'")
        .arg_from_usage("-q, --queue[some] 'Rabbit's queue name'")
        .arg_from_usage("-t, --timeout[5 sec] 'Heartbeat timeout'")
        .arg_from_usage("-T, --sleep[0 msec] 'Sleep between publish'")
        .arg_from_usage("-C, --count[999999] 'Process n messages'")
        .arg_from_usage("-r, --read 'Read messages from dir'")
        .arg_from_usage("-P, --parallel[1] 'Parallel run'");
    let matches = app.clone().get_matches();

    let prm = Opt {
        parall: value_t!(matches, "parallel", usize).unwrap_or(1),
        timeout: value_t!(matches, "timeout", u16).unwrap_or(5),
        count_messages: value_t!(matches, "count", u32).unwrap_or(999999),
        dcount: digicount(value_t!(matches, "count", u32).unwrap_or(999999)),
        sleep: value_t!(matches, "sleep", u64).unwrap_or(0),
        queue: value_t!(matches, "queue", String).unwrap_or("some".to_string()),
        is_add: match matches.is_present("add") {
            true => true,
            _ => false,
        },
        is_read: match matches.is_present("read") {
            true => true,
            _ => false,
        },
        save: match matches.is_present("save-file") {
            true => true,
            _ => false,
        },
        queue_options: QueueDeclareOptions {
            durable: true,
            ..Default::default()
        },
    };

    let mut children = vec![];

    if prm.parall > 1 {
        println!("do it in {} parallel", prm.parall);
    }
    for _  in 0..prm.parall {
        if matches.is_present("consumer") {
            println!("spawn consumer");
            let prm = prm.clone();
            children.push(std::thread::spawn(|| {
                consumer::run(prm);
            }));
        };

        if matches.is_present("publisher") {
            println!("spawn publisher");
            let prm = prm.clone();
            children.push(std::thread::spawn(move || {
                publisher::run(prm.clone());
            }));
        };
    }

    for child in children {
        let _ = child.join();
    }
}
