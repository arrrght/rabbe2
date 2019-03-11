#!/bin/bash
set -e

if [ -z "$CLUSTER_WITH" -o "$CLUSTER_WITH" = "$hostname" ]; then
  echo "Running as single server"
	/usr/local/bin/docker-entrypoint.sh rabbitmq-server
else
  echo "Running as clustered server"
	/usr/local/bin/docker-entrypoint.sh rabbitmq-server -detached
  rabbitmqctl stop_app

  echo "Joining cluster $CLUSTER_WITH"
  rabbitmqctl join_cluster rabbit@$CLUSTER_WITH

	pid=`pidof beam.smp`
	echo "PID:$hostname $pid"

	rabbitmqctl stop
	echo -n "Waiting for rabbitmq stop.. "
	while ps -p $pid > /dev/null; do sleep 1; done;
	echo "ok.\nRestarting RabbitMQ"
	/usr/local/bin/docker-entrypoint.sh rabbitmq-server
fi
