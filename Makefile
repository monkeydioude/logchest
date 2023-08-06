all: image-build run

run:
	docker run -it -p 8081:8081 --rm --name logchest-run logchest

image-build:
	docker build -t logchest .

build:
	rm -rf "$(PWD)/target/debug/logchest"
	docker run --rm --user "$(id -u)":"$(id -g)" -v "$(PWD)":/usr/src/logchest -w /usr/src/logchest rust:latest cargo build
