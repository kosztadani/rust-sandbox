# Rust Sandbox

This is a sandbox repository to experiment with Rust

## hello-world

A somewhat unusual implementation of the "Hello, World!" program.

It needs a nightly compiler and is only targeted at x64 Linux.

### Building

```bash
cd hello-world
cargo build --release
```

### Running

```bash
hello-world/release/rust-sandbox-hello-world
```

## tcp-example

This is a simple example application where a server and client communicate
over a custom TCP protocol.

The client can send a "math request" (an opcode and two operands), to which
the server replies with an answer.

I have similar programs in my [`java-sandbox`][java-sandbox] repository, and
also a Wireshark dissector for this protocol in my
[`wireshark-plugin-sandbox`][wireshark-plugin-sandbox] repository.

[java-sandbox]: https://github.com/kosztadani/java-sandbox

[wireshark-plugin-sandbox]: https://github.com/kosztadani/wireshark-plugin-sandbox

### Building

```bash
cd tcp-example
cargo build --release
```

### Running the server

```bash
tcp-example/target/release/server 9999
```

### Running the client

```bash
tcp-example/target/release/client localhost 9999
```

