all: image-build run

run:
	docker run -it --network="host" --rm --name logchest-run logchest

image-build:
	docker build -t logchest .

build:
	docker run --rm --network="host" --user "$(id -u)":"$(id -g)" -v "$(PWD)":/usr/src/logchest -w /usr/src/logchest rust:latest cargo build --release