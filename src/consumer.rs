use crate::lapin::channel::BasicConsumeOptions;
use crate::lapin::client::ConnectionOptions;
use crate::lapin::types::FieldTable;
//use clap::ArgMatches;
use failure::Error;
use futures::{future::Future, Stream};
use lapin_futures as lapin;
//use serde_json::Value;
use std::io::{self, Write};
use tokio;
use tokio::net::TcpStream;
use tokio::runtime::Runtime;
//use std::cell::RefCell;
//use std::rc::Rc;
use std::fs::File;
use std::sync::{Arc, Mutex};

pub fn run(prm: super::Opt) {
    println!("run consumer with timeout: {}", prm.timeout);
    let arc_prm = Arc::new(Mutex::new(prm));
    let cloned2_arc_prm = arc_prm.clone();
    //let rc_prm = RefCell::new(prm);
    //let count_messages = RefCell::new(prm.count_messages);
    //let p_padding = RefCell::new(prm.dcount);
    //let to_queue = RefCell::new(prm.save_queue.clone());
    let counter = Arc::new(Mutex::new(0u32));

    let addr = "127.0.0.1:5672".parse().unwrap();

    Runtime::new()
        .unwrap()
        .block_on_all(
            TcpStream::connect(&addr)
                .map_err(Error::from)
                .and_then(move |stream| {
                    lapin::client::Client::connect(
                        stream,
                        ConnectionOptions {
                            username: super::RBT_USER.to_string(),
                            password: super::RBT_PASSWORD.to_string(),
                            heartbeat: cloned2_arc_prm.lock().unwrap().clone().timeout,
                            ..Default::default()
                        },
                    )
                    .map_err(Error::from)
                })
                .and_then(|(client, heartbeat)| {
                    tokio::spawn(heartbeat.map_err(|m| {
                        println!("H: {:?}", m);
                    }));

                    client.create_channel().map_err(Error::from)
                })
                .and_then(|channel| {
                    let id = channel.id;
                    println!("created channel with id: {}", id);

                    let q_str = arc_prm.lock().unwrap().clone().queue.clone();
                    let ch = channel.clone();
                    let cloned_arc_prm = arc_prm.clone();
                    let queue_options = arc_prm.lock().unwrap().clone().queue_options;
                    channel
                        .queue_declare(&q_str, queue_options, FieldTable::new())
                        .and_then(move |queue| {
                            println!(
                                "channel {} declared queue {}",
                                id,
                                cloned_arc_prm.lock().unwrap().clone().queue
                            );
                            channel.basic_consume(
                                &queue,
                                //&arc_prm.lock().unwrap().clone().queue,
                                "rust_consumer",
                                BasicConsumeOptions::default(),
                                FieldTable::new(),
                            )
                        })
                        .and_then(move |stream| {
                            println!("got consumer stream");

                            let cloned_arc_prm = arc_prm.clone();
                            stream.for_each(move |message| {
                                //let is_save = RefCell::new(prm.save);

                                if cloned_arc_prm.lock().unwrap().clone().save {
                                    let mut cnt = counter.lock().unwrap();
                                    let count_messages =
                                        arc_prm.lock().unwrap().clone().count_messages;
                                    //let count_messages = count_messages.clone().into_inner();
                                    *cnt += 1;
                                    if *cnt > count_messages {
                                        println!("\nDONE");
                                        std::process::exit(0);
                                    }
                                    let p_padding = arc_prm.lock().unwrap().clone().dcount;
                                    let f_name = format!("messages/{:0w$}", cnt, w = p_padding);
                                    //let f_name = "messages/".to_string() + &cnt.to_string();
                                    let mut file = File::create(f_name).unwrap();
                                    file.write_all(&message.data).unwrap();
                                    print!("s");
                                    io::stdout().flush().expect("flushed");
                                } else {
                                    //let data = String::from_utf8(message.data).unwrap();
                                    //let v: Value = serde_json::from_str(&data).unwrap();
                                    //print!("{}", v["head"]["request"]["special"].as_str().unwrap());
                                    print!("r");
                                    io::stdout().flush().expect("flushed");
                                }
                                ch.basic_ack(message.delivery_tag, false)
                            })
                        })
                        .map_err(Error::from)
                }),
        )
        .expect("runtime failure");
}
