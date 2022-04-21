FROM ubuntu:focal

ENV PATH=$PATH:/root/.cargo/bin

WORKDIR /usr/src/app

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        build-essential \
        ca-certificates \
        wget \
    && wget https://sh.rustup.rs -O rustup.sh \
    && chmod +x ./rustup.sh \
    && ./rustup.sh -y \
    && rm -rf ./rustup.sh /var/lib/apt/lists/* \
    && apt-get remove -y \
        wget \
    && apt-get autoremove -y --purge \
    && apt-get clean -y
