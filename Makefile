.PHONY: clean
clean:
	rm -rf docker/mysql
.PHONY: build
build:
	cargo build
.PHONY: up
up:
	docker-compose up
.PHONY: down
down:
	docker-compose down
.PHONY: run
run:
	cargo run
