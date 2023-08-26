# A simple Blockchain implementation

Rust reimplementation of [dvf/blockchain](https://github.com/dvf/blockchain).
This is the source code on [Building a Blockchain](https://medium.com/p/117428612f46).

## Getting Started

### 1. Install Rust toolchain

```
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Clone this repository and enter the directory

```
$ git clone https://github.com/whtsht/blockchain-rs.git
$ cd blockchain-rs
```

### 3. Run the server

Start on default port number 8080.

```
$ cargo run
```

### 4. Add more instances

To add more instance, specify the port number.

```
$ cargo run 8081
$ cargo run 8082
$ cargo run 8083
```
