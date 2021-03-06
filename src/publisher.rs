use crate::lapin::channel::{BasicProperties, BasicPublishOptions};
use crate::lapin::client::ConnectionOptions;
use crate::lapin::types::FieldTable;
use failure::Error;
use futures::future::Future;
use lapin_futures as lapin;

use tokio;
use tokio::net::TcpStream;
use tokio::runtime::Runtime;

//use clap::ArgMatches;
use handlebars::Handlebars;
use log::info;
use std::collections::BTreeMap;
use std::fs;
use std::fs::read_to_string;
use std::io::{self, Write};
use std::net::SocketAddr;

pub fn run(prm: super::Opt) {
    if prm.is_read {
        for entry in fs::read_dir("./messages/").unwrap() {
            let f_name = entry.unwrap().path();
            let contents = read_to_string(f_name).unwrap();
            let addr = "127.0.0.1:5672".parse().unwrap();
            connect_to(addr, &contents, prm.clone());
        }
    } else {
        println!("runing publisher");
        if prm.is_add {
            let json = r#"{
          "header": {
            "source": "pledge",
            "queueQuery": {
              "vin": "{{vin}}"
            }
          }"#;
            #[rustfmt::skip]
            let vins = [
                "JN1TDNS51U0482186", "XTA219120JY275047", "XTA219470H0119223", "XTAGAB320J1051431",
                "XTA212140J2315419", "XTAGFLA10JY187096", "XTAKS045LJ1122567", "XTAGFLA10JY187178",
                "XTAGFL330KY216026", "XTAGFL330JY184955", "XTAGFL320JY203421", "XTAGAB330J1071323",
                "XTAGAB330H1024974", "XTA212140H2291501", "XTAGFL110JY214769", "XTAGAB330J1071325",
                "Z0X219059JS009306", "Z0X219059JS009318", "Z0X219059JS009339", "Z0X219259JS002792",
                "XTA219120JY275047", "XLRTE47MS0E799266", "1J4RR5GG3BC603050", "1J4RR5GG3BC603050",
                "XW7BN4FK50S108539", "JTMCV05J904119304", "Z94CB51ABHR084092", "Z94G2811AJR077116",
                "VF7RD5FNAAL527214", "XW8ZZZ7PZHG001658",
            ];
            let mut reg = Handlebars::new();
            println!("go fk ur slf");
            reg.register_template_string("t1", json).unwrap();
            for x in vins.iter() {
                let mut data = BTreeMap::new();
                data.insert("vin".to_string(), x.to_string());
                let out = format!("{}", reg.render("t1", &data).unwrap());
                let addr = "127.0.0.1:5672".parse().unwrap();
                connect_to(addr, &out, prm.clone());
                io::stdout().flush().unwrap();
            }
        } else {
            for _x in 1..prm.count_messages {
                let addr = "127.0.0.1:5672".parse().unwrap();
                connect_to(addr, super::RBT_MESSAGE, prm.clone());
                io::stdout().flush().unwrap();
                std::thread::sleep(std::time::Duration::from_millis(prm.sleep));
            }
        }
    }
}

fn connect_to(addr: SocketAddr, msg: &str, prm: super::Opt) {
    let msg2 = msg.clone().as_bytes().to_vec();
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
                .and_then(move |channel| {
                    let id = channel.id;
                    info!("channel {} created", id);

                    let string_options = FieldTable::new();
                    //string_options.insert(
                    //    "x-message-ttl".to_string(),
                    //    lapin::types::AMQPValue::LongUInt(300000),
                    //);

                    channel
                        .queue_declare(
                            &prm.queue,
                            prm.clone().queue_options,
                            string_options,
                        )
                        .and_then(move |_| {
                            //println!("channel {} declare queue {}", id, "hello");
                            let p = channel.basic_publish(
                                "",
                                &prm.queue,
                                msg2,
                                BasicPublishOptions::default(),
                                BasicProperties::default(),
                            );
                            print!(".");
                            io::stdout().flush().expect("flushed");
                            p
                        })
                        .map_err(Error::from)
                }),
        )
        .expect("runtime failure");
}
