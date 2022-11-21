FROM rust:buster as builder

WORKDIR /app
COPY . /app

RUN cargo build --release


# =============
FROM phusion/baseimage:focal-1.2.0

LABEL maintainer="bingryan@outlook.com"

COPY --from=builder /app/target/release/eclub /usr/local/bin


RUN useradd -m -u 1000 -U -s /bin/sh -d /eclub eclub && \
    mkdir -p /data && \
    chown -R eclub:eclub /data && \
# unclutter and minimize the attack surface
	rm -rf /usr/bin /usr/sbin /usr/share/man /usr/lib/python*


USER eclub

EXPOSE 9988
VOLUME ["/data"]

ENTRYPOINT ["/usr/local/bin/eclub"]