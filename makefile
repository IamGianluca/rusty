-include .env  # if .env file exists, include it

install_diesel_cli:
	sudo apt-get install libsqlite3-dev libmysqlclient-dev libpq-dev -y && \
	cargo install diesel_cli

start_db: 
	docker pull postgres:16 && \
	docker run --name $(POSTGRES_CONTAINER_NAME) -e POSTGRES_PASSWORD=$(POSTGRES_PASSWORD) -d -p 5432:5432 postgres

attach_db:
	psql -h localhost -U $(POSTGRES_USER) -d $(DATABASE_NAME)

stop_db:
	docker container stop $(POSTGRES_CONTAINER_NAME) && \
	docker container rm $(POSTGRES_CONTAINER_NAME)
	
prepare_db:
	diesel setup && \
	diesel migration run 

test: 
	cargo test -- --test-threads=1

