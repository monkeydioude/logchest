all: build run

run:
	docker run -it --rm --name logchest-run logchest

build:
	docker build -t logchest .