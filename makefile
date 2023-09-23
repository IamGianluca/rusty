prepare_ci_db:
	cargo install diesel_cli && \
	diesel setup && \
	diesel migration run 

test: 
	cargo test -- --test-threads=1
