
# Creating a Load Balancer in Rust

This is a Load Balancer implemented in Rust for learning purposes. A load balancer routes client requests across multiple servers, ensuring efficient distribution, high availability, and flexibility to adapt to server changes.

## About Load Balancers

A load balancer is a crucial component in managing network traffic efficiently. It performs several functions:

- Distributes client requests/network load efficiently across multiple servers.
- Ensures high availability and reliability by sending requests only to online servers.
- Provides the flexibility to add or subtract servers as demand dictates.

### Key Functions

- Balancing incoming client requests among backend servers.
- Health checks to ensure server availability.
- Handling server failures and re-routing traffic when servers go offline.
- Re-activating servers when they come back online.

## Acknowledgments

This project was inspired by the concept of load balancing in network systems. It's a learning project that explores the fundamentals of creating a load balancer from scratch.

[![Rust](https://www.rust-lang.org/logos/rust-logo-32x32.png)](https://www.rust-lang.org/)

This implementation uses Rust, a systems programming language known for its memory safety and low-level control over hardware.

## Getting Started

To run the load balancer, navigate to the `src` folder and execute the following command:

```bash
cargo run -- IP:port1,IP:port2,...,IP:portN
```

- `IP:port1,IP:port2,...,IP:portN`: Comma-separated list of server IP addresses and ports.

### Test Scripts
- You need Nodejs installed on your system.

- To spawn test servers, use the following command in the `scripts` folder:

```bash
node spawnTestServers.js startServers N
```

- To test concurrency, use the following command in the `scripts` folder:

```bash
node testConcurency.js N
```
- This defaults to 10.

## Contributors

- [skyn37](https://github.com/yourusername) - Initial work

Feel free to contribute, report issues, and make suggestions. 

⚠️ Please note that this load balancer is not intended for production use. It's designed for educational purposes, specifically learning Rust.
