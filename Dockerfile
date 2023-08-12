FROM rust:latest

WORKDIR /usr/src/logchest
COPY ./Cargo.lock ./Cargo.toml ./
COPY ./src ./src/

EXPOSE 8081

RUN cargo install --path .
CMD ["logchest"]