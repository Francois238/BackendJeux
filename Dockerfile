FROM rust:1.69.0 as build

WORKDIR /app
COPY Cargo.* /app/
COPY src/ /app/src

RUN cargo build --release

FROM ubuntu:23.04

RUN  apt-get update \
  && apt-get install -y libpq-dev openssl wget\
    && wget http://nz2.archive.ubuntu.com/ubuntu/pool/main/o/openssl/libssl1.1_1.1.1f-1ubuntu2_amd64.deb \
    && dpkg -i libssl1.1_1.1.1f-1ubuntu2_amd64.deb \
    && apt-get remove -y wget \
  && rm -rf /var/lib/apt/lists/* \
  && useradd -m runner 

COPY --from=build /app/target/release/BackendJeux /home/runner/app/BackendJeux

RUN chown -R runner:runner /home/runner/app/

EXPOSE 8080

USER runner

WORKDIR /home/runner/app/

ENTRYPOINT ["./BackendJeux"]
