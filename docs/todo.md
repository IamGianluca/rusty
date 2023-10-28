## Todo

#### Features

- [x] Client can sign up via HTTP request to Server 
- [x] Client can login/authenticate via HTTP request to Server
- [ ] Before any operation, check that the user is authenticated (either a list of authorized users or an instance variable on the User struct)
- [x] Client can update credentials via HTTP request to Server
- [x] Client can join a public channel via HTTP request to Server
- [x] Client can create new channel via HTTP request to Server
- [ ] Only authorized user can create new channel via HTTP request to Server
- [x] Client can send a message in a channel via HTTP request to Server
- [x] Only authorized user can send a message in a channel via HTTP request to Server
- [ ] Client receives new messages to a channel they are subscribed via WebSocket from Message Server

#### Refactoring

- [ ] Use connection pool to connect to Postgres DB
- [ ] The creation of a new user and password should happen within the same db transaction
- [ ] Improve REST API design (e.g., passing values via path)

#### Testing

- [ ] Integration tests for `ChannelRepository`
- [ ] Create in-memory version of `UserRepository` and `ChannelRepository` to speed up test suite

#### Load Testing

- [ ] Benchmark transactions/second (using sending a message as the transaction unit)
