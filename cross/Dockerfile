#FROM rustembedded/cross:armv7-unknown-linux-gnueabihf-0.2.1
FROM rustembedded/cross:aarch64-unknown-linux-gnu

RUN dpkg --add-architecture arm64 && \
    apt-get update && \
    apt-get install --assume-yes libpq-dev:arm64