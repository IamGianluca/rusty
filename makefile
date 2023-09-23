-include .env  # if .env fine exists, include it

install_diesel_cli:
	sudo apt-get install libsqlite3-dev libmysqlclient-dev libpq-dev -y && \
	cargo install diesel_cli

start_db: 
	docker pull postgres && \
	docker run --name rusty_db -e POSTGRES_PASSWORD=$(POSTGRES_PASSWORD) -d -p 5432:5432 postgres

attach_db:
	psql -h localhost -U postgres -d $(DATABASE_NAME)

stop_db:
	docker container stop rusty_db && \
	docker container rm rusty_db
	
prepare_db:
	diesel setup && \
	diesel migration run 

test: 
	cargo test -- --test-threads=1

