FROM rust:latest

WORKDIR /usr/src/logchest
COPY . .

EXPOSE 8080

RUN cargo install --path .
CMD ["logchest"]