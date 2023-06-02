FROM rust:latest

WORKDIR /usr/src/logchest
COPY . .

EXPOSE 8000

RUN cargo install --path .
CMD ["logchest"]