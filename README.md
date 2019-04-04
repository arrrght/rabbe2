[![Travis Build Status][travis-badge]][travis-url]

[travis-badge]: https://travis-ci.org/arrrght/rabbe2.svg?branch=master
[travis-url]: https://travis-ci.org/arrrght/rabbe2

## Тесты на проверку соединения/отвал с приложения на RabbitMQ

docker-compose, конфиги находятся в каталоге 2rabbits

Два(или больше) раббита поднимаются из докера, перед ними - HAProxy

Затем поднимается "приложение" -  rabbe2, которое постоянно пишет(publisher) и читает(consumer) из очереди

publisher - постоянно пересоздает соединение

consumer - держит, но падает по таймауту в случае переключения мастер-ноды rabbit(что нормально)

### Развлечения:
 - на одной консольке запускаем ```docker-compose up -d```
 - на другой консольке роняем/поднимаем сервисы: ```docker-compose kill rabbit0``` и ```docker-compose start rabbit0```
 - в результате publisher-у все равно, он пересоздает соединение каждый раз
 - consumer должен упать по тайм-ауту ```rabbe consumer --timeout 5```, что в простейшем случае нормально, поскольку его дролжен перезапустить rancher/kubernates
 - consumer не должен падать, а должен пересоздавать соединение (ещё не сделано)

### Remarks
build'n'run:
```
cargo run -- -C99 -P4 -cos -h

rabbe2 

USAGE:
    rabbe2 [FLAGS] [OPTIONS]

FLAGS:
    -a, --add          Add some messages to queue
    -c, --consumer     Run consumer
    -h, --help         Prints help information
    -p, --publisher    Run publisher
    -r, --read         Read messages from dir
    -s, --save-file    Save messages to file into subdir messages
    -V, --version      Prints version information

OPTIONS:
    -C, --count <99999>      Process n messages
    -P, --parallel <1>       Parallel run
    -q, --queue <some>       Rabbit's queue name
    -T, --sleep <0 msec>     Sleep between publish
    -t, --timeout <5 sec>    Heartbeat timeout
```
