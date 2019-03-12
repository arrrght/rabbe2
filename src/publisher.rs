use lapin_futures as lapin;
use crate::lapin::channel::{BasicProperties, BasicPublishOptions, QueueDeclareOptions};
use crate::lapin::client::ConnectionOptions;
use crate::lapin::types::FieldTable;
use failure::Error;
use futures::future::Future;

use tokio;
use tokio::net::TcpStream;
use tokio::runtime::Runtime;

use log::info;
use std::net::SocketAddr;
use std::io::{self, Write};
use clap::{ArgMatches};


pub fn run(_args: &ArgMatches) {
    println!("runing publisher");
    loop {
        let addr = "127.0.0.1:5672".parse().unwrap();
        connect_to(addr);
        io::stdout().flush().expect("flushed");
        std::thread::sleep(std::time::Duration::from_millis(super::SLEEP_MILLIS));
    }
}

fn connect_to(addr: SocketAddr) {
    Runtime::new()
        .expect("died on runtime-new")
        .block_on_all(
            TcpStream::connect(&addr)
                .map_err(Error::from)
                .and_then(|stream| {
                    lapin::client::Client::connect(
                        stream,
                        ConnectionOptions {
                            username: super::RBT_USER.to_string(),
                            password: super::RBT_PASSWORD.to_string(),
                            ..Default::default()
                        },
                    )
                    .map_err(Error::from)
                })
                .and_then(|(client, _)| client.create_channel().map_err(Error::from))
                .and_then(|channel| {
                    let id = channel.id;
                    info!("channel {} created", id);

                    let mut string_options = FieldTable::new();
                    //string_options.insert(
                    //    "x-message-ttl".to_string(),
                    //    lapin::types::AMQPValue::LongUInt(300000),
                    //);

                    channel
                        .queue_declare(
                            super::RBT_QUEUE,
                            QueueDeclareOptions {
                                //durable: true,
                                ..Default::default()
                            },
                            string_options,
                        )
                        .and_then(move |_| {
                            info!("channel {} declare queue {}", id, "hello");
                            let pika_pika = super::RBT_MESSAGE.clone().as_bytes().to_vec();
                            let p = channel.basic_publish(
                                "",
                                super::RBT_QUEUE,
                                pika_pika,
                                BasicPublishOptions::default(),
                                BasicProperties::default(),
                            );
                            print!(".");
                            p
                            //futures::future::ok(true)
                        })
                        .map_err(Error::from)
                }),
        )
        .expect("runtime failure");
}
