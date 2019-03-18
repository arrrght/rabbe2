use crate::lapin::channel::{BasicConsumeOptions, QueueDeclareOptions};
use crate::lapin::client::ConnectionOptions;
use crate::lapin::types::FieldTable;
use clap::{ArgMatches};
use failure::Error;
use futures::{future::Future, Stream};
use lapin_futures as lapin;
use serde_json::Value;
use std::io::{self, Write};
use tokio;
use tokio::net::TcpStream;
use tokio::runtime::Runtime;

pub fn run(_args: &ArgMatches, prm: super::Opt) {

    println!("run consumer with timeout: {}", prm.timeout);
    let timeout = prm.timeout;

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
                            heartbeat: timeout,
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
                .and_then(move |channel| {
                    let id = channel.id;
                    println!("created channel with id: {}", id);

                    let ch = channel.clone();
                    channel
                        .queue_declare(
                            &prm.queue,
                            QueueDeclareOptions::default(),
                            FieldTable::new(),
                        )
                        .and_then(move |queue| {
                            println!("channel {} declared queue {}", id, prm.queue);
                            channel.basic_consume(
                                &queue,
                                "rust_consumer",
                                BasicConsumeOptions::default(),
                                FieldTable::new(),
                            )
                        })
                        .and_then(|stream| {
                            println!("got consumer stream");

                            stream.for_each(move |message| {
                                let data = String::from_utf8(message.data).unwrap();
                                let v: Value = serde_json::from_str(&data).unwrap();
                                print!("{}", v["head"]["request"]["special"].as_str().unwrap());
                                io::stdout().flush().expect("flushed");
                                ch.basic_ack(message.delivery_tag, false)
                            })
                        })
                        .map_err(Error::from)
                }),
        )
        .expect("runtime failure");
}
