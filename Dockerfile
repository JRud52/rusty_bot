FROM centos:7
MAINTAINER Justin Rhude <justin.rhude52@gmail.com>

EXPOSE 8080

ENV SOURCES=/SOURCES

RUN yum update -y
RUN yum install -y file gcc openssl-devel
RUN curl -sSf https://static.rust-lang.org/rustup.sh | sh -s -- --channel=nightly --disable-sudo

RUN mkdir -p $SOURCES
ADD ./ $SOURCES

WORKDIR $SOURCES
RUN cargo build --release --verbose

CMD ./target/release/rusty_bot
