# `pterm`

> Sends SIGTERM to a given process by name. Think of it as a diet coke version of `pgrep Python | xargs kill -SIGTERM` 

### Usage

```bash
$ pterm <process_name>
```
### Development

- `cargo run <process_name>`

#### Requirements

- [rust](https://www.rust-lang.org/)

#### Building

- `cargo build` # debug version
- `cargo build --release` # final version