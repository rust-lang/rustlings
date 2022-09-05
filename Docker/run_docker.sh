#!/bin/sh

HOST_PATH=$PWD/..

# store a command history inside containers into this file

touch $HOME/.rustlings_history

xhost local:

sudo docker run --rm \
-v /tmp/.X11-unix/:/tmp/.X11-unix \
-v $HOST_PATH:/mnt/rustlings \
-v $HOME/.rustlings_history:/root/.bash_history \
-w /mnt/rustlings \
--sysctl fs.mqueue.msg_max=128 \
--ulimit msgqueue=8192000 \
--ulimit core=-1 \
--shm-size=12G \
--name rustlings \
-i -t rustlings:latest \
/bin/bash
