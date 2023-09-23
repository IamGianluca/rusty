### Rusty 

`rusty` is a pet project aimed at learning Rust. The scope of this project is to create the backend for a simple instant messaging app, like Slack or Discord.

### How to use

### How to contribute

This is really a pet project to learn Rust, so I'm not expecting anyone to be seriously interested in contributing. That said, if you are, please respect this naming convention to tag your commits:

* API relevant changes:
    * `feat`: Commits that adds a new feature
    * `fix`: Commits that fixes a bug
* `refactor`: Commits that rewrite/restructure your code, however does not change any behaviour
    * `perf`: Special `refactor` commits, that improve performance
* `style`: Commits that do not affect the meaning (white-space, formatting, missing semi-colons, etc.)
* `test`: Commits that add missing tests or correcting existing tests
* `docs`: Commits that affect documentation only
* `build`: Commits that affect build components like build tool, CI pipeline, dependencies, project version, ...
* `ops`: Commits that affect operational components like infrastructure, deployment, backup, recovery, ...
* `chore`: Miscellaneous commits e.g. modifying `.gitignore`

### TODO

- [ ] Web Framework: That can be used by clients to communicate with the backend server
- [ ] Message Server: Listens to updates from the backend server. Fans out real-time events to all clients (within a workspace)
- [ ] Load testing
