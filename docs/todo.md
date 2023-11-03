## Todo

#### Features

- [x] Client can sign up via HTTP request to Server 
- [x] Client can login/authenticate via HTTP request to Server, and receive JWT
- [x] Client can update credentials via HTTP request to Server
- [x] Client can join a public channel via HTTP request to Server
- [x] Client can create a new channel via HTTP request to Server
- [x] Client can send a message in a channel via HTTP request to Server
- [x] Before any operation, server verifies JWT auth
- [ ] Client gets notified of new messages to channel they are subscribed via WebSocket from Message Server

#### Refactoring

- [ ] Use connection pool to connect to Postgres DB
- [ ] The creation of a new user and password should happen within the same db transaction
- [ ] Improve REST API design (e.g., passing some values via path)

#### Testing

- [ ] Integration tests for `ChannelRepository`
- [ ] Create in-memory version of `UserRepository` and `ChannelRepository` to speed up test suite

#### Load Testing

- [ ] Benchmark transactions/second (using sending a message as the transaction unit)
