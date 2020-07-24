.PHONY: clean
clean:
	rm -rf docker/mysql
.PHONE: build
build:
	cargo build
.PHONE: up
up:
	docker-compose up
.PHONE: down
down:
	docker-compose down
.PHONE: run
run:
	cargo run
