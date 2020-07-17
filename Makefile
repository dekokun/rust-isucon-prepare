.PHONY: clean
clean:
	rm -rf docker/mysql
.PHONE: build
build:
	docker-compose build
.PHONE: up
up:
	docker-compose up
