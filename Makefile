all: image-build run

run:
	docker run -it -p 8081:8081 --rm --name logchest drannoc/logchest

image-build:
	docker build -t drannoc/logchest .

docker-push: check-env
	docker login -u $(DOCKER_USER) --password $(DOCKER_PASSWD)
	docker push drannoc/logchest

update-remote: image-build docker-push

build:
	rm -rf "$(PWD)/target/debug/logchest"
	docker run --rm --user "$(id -u)":"$(id -g)" --network="host" -v "$(PWD)":/usr/src/logchest -w /usr/src/logchest rust:latest cargo build

check-env:
ifndef DOCKER_USER
	$(error DOCKER_USER is undefined)
endif
ifndef DOCKER_PASSWD
	$(error DOCKER_PASSWD is undefined)
endif