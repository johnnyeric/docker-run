#!/bin/bash

export SERVER_LISTEN_ADDR="127.0.0.1"
export SERVER_LISTEN_PORT="8088"
export SERVER_WORKER_THREADS="10"

export API_ACCESS_TOKEN="magmatic-handyman-confirm-cauldron"

export DOCKER_UNIX_SOCKET_PATH="/Users/pii/Library/Containers/com.docker.docker/Data/docker.raw.sock"
export DOCKER_UNIX_SOCKET_READ_TIMEOUT="3"
export DOCKER_UNIX_SOCKET_WRITE_TIMEOUT="3"

export DOCKER_CONTAINER_HOSTNAME="glot"
export DOCKER_CONTAINER_USER="glot"
export DOCKER_CONTAINER_MEMORY="500000000"
export DOCKER_CONTAINER_ULIMIT_NOFILE_SOFT="90"
export DOCKER_CONTAINER_ULIMIT_NOFILE_HARD="100"
export DOCKER_CONTAINER_ULIMIT_NPROC_SOFT="90"
export DOCKER_CONTAINER_ULIMIT_NPROC_HARD="100"
export DOCKER_CONTAINER_CAP_DROP="MKNOD NET_RAW NET_BIND_SERVICE"

export RUN_MAX_EXECUTION_TIME="10"
export RUN_MAX_OUTPUT_SIZE="100000"

export RUST_LOG=debug

cargo run
