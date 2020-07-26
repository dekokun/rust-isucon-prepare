.PHONY: clean
clean:
	rm -rf docker/mysql
.PHONY: build
build:
	DATABASE_URL=mysql://isucon:isucon@localhost:3306/isucon cargo build
.PHONY: up
up:
	docker-compose up
.PHONY: down
down:
	docker-compose down
.PHONY: run
run:
	DATABASE_URL=mysql://isucon:isucon@localhost:3306/isucon cargo run
.PHONY: prepare
prepare:
	mysql -uisucon -pisucon -h127.0.0.1 -P3306 -e'drop table if exists isucon.payment;'
	mysql -uisucon -pisucon -h127.0.0.1 -P3306 -e"CREATE TABLE isucon.payment ( customer_id int not null, amount int not null, account_name text);"
