### Rusty 

<img src="./imgs/logo.png" width=300 height=300>

`rusty` is a pet project aimed at learning Rust. The scope of this project is to create the backend for a simple, instant messaging app like Slack or Discord.

Some features:
* Written in Rust for blazing fast performance
* Leverages one of the fastest HTTP web server in Actix Web
* Uses Domain-Driven Design for a clear codebase that is easily extendible
* Secure authentication with JSON Web Tokens

### Installation

Before starting the service, we must create a `.env` file. The `.env` file must contain the following environment variables.

```
DATABASE_URL=
JWT_SECRET_KEY=
POSTGRES_USER=
POSTGRES_PASSWORD=
DATABASE_NAME=rusty
POSTGRES_CONTAINER_NAME=rusty_db
```

Then, we must start the database and apply all migrations.

```bash
make start_db 
make prepare_db
```

We can then start the service with the following command:

```bash
cargo run
```

We will be able to start sending HTTP requests to the service using this format:

```bash
curl -X POST -H "Content-Type: application/json" -d '{"name": "channel name", "description": "test channel"}' http://127.0.0.1:8080/channel
```

### Directory Structure

* `src/adapters`: Adapters and repository classes.
* `src/domain`: Domain model.
* `src/service_layer`: Service layer.
* `src/lib.rs`: REST API.
* `src/main.rs`: The entry point to launch the app.


### Development

`rusty` is a pet project to learn Rust; thus, I do not expect external contributions. Yet, if you want to help, please respect this naming convention to tag your commits:

* API relevant changes:
    * `feat`: Commits that add a new feature
    * `fix`: Commits that fixes a bug
* `refactor`: Commits that rewrite/restructure your code but does not change any behavior
    * `perf`: Special `refactor` commits that improve performance
* `style`: Commits that do not affect the meaning (white space, formatting, missing semi-colons, etc.)
* `test`: Commits that add missing tests or correct existing tests
* `docs`: Commits that affect documentation only
* `build`: Commits that affect build components like build tool, CI pipeline, dependencies, project version, etc...
* `ops`: Commits that affect operational components like infrastructure, deployment, backup, recovery, ...
* `chore`: Miscellaneous commits e.g., modifying `.gitignore`

Before we start, we must install `diesel_cli` to help manage the project. If you're using Ubuntu, you can do that by running the following command:

```bash
make install_diesel_cli
```

Then, we must start the app database and apply all migrations to get the database to the desired state.

```bash
make start_db 
make prepare_db
```

We should now be able to run unit and integration tests.

```bash
make test
```

To stop the service, we can stop the Docker container running Postgres.

```bash
make stop_db
```

There are other helper commands in the `makefile` file that can be useful while contributing to this project.

### Tools

- Docker
- [Cargo](https://github.com/rust-lang/cargo)
- [Diesel](https://github.com/diesel-rs/diesel)
- [PostgreSQL](https://github.com/postgres/postgres)
- [Actix Web](https://github.com/actix/actix-web)
