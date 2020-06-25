FROM debian:buster-slim

RUN apt-get update && \
    apt-get install -y \
    wget binfmt-support qemu-user-static ssh

ENTRYPOINT /bin/bash