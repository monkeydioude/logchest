FROM rust:alpine3.17

WORKDIR /usr/src/logchest
COPY . .

RUN cargo install --path .
CMD ["logchest"]