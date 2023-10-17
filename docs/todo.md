## Todo

#### Features

- [ ] Client can sign up via HTTP request to Server 
- [ ] Client can login/authenticate via HTTP request to Server
- [ ] Client can update credentials via HTTP request to Server
- [ ] Client can join a public channel via HTTP request to Server
- [ ] Client can create new channel via HTTP request to Server
- [ ] Client can send a message in a channel via HTTP request to Server
- [ ] Client receives new messages to a channel they are subscribed via WebSocket from Message Server

#### Refactoring

- [ ] The creation of a new user and password should happen within the same db transaction

#### Testing

- [ ] Integration tests for `MessageRepository`
- [ ] Create in-memory version of `UserRepository` and `MessageRepository` to speed up test suite

#### Load Testing

- [ ] Benchmark transactions/second (using sending a message as the transaction unit)