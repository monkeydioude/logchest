all: image-build run

run:
	docker run -it -p 8080:8080 --rm --name logchest-run logchest

image-build:
	docker build -t logchest .

build:	
	docker run --rm --user "$(id -u)":"$(id -g)" -v "$(PWD)":/usr/src/logchest -w /usr/src/logchest rust:latest cargo build --release