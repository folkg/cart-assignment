# furniture-store

A simple React front-end to display a functional and responsive shopping cart for a furniture store. This is a small toy app with both a Node.js / Express.js server and a Rust Axum server for comparison and benchmarking.

## Benchmark Results

The Rust Axum server performed notably better than the Node Express.js server. It outperformed Node by 328% in requests per second and mean latency, and an impressive 429% in throughput. In addition, the Rust server consumed 88% less memory compared to Node.

The results are summarized in the table below. The full output from the Apache ab benchmarking tool is available in the `benchmark` directory.

| Server          | Requests per second | Mean Latency (ms) | Throughput (KBps) | Memory Consumption (MB) |
| --------------- | ------------------- | ----------------- | ----------------- | ----------------------- |
| Node Express.js | 4235.34             | 23.611            | 1000.93           | 97.5                    |
| Rust Axum       | 13904.43            | 7.192             | 4195.77           | 11.7                    |

## Instructions for Running

Note that the Node server and the Rust server are separate and independent. The React app can be run with either server. Both servers are configured to run on port 4000 by default, so only one server can be run at a time.

1. Clone the repository

### Client

1. cd into the `client` directory and run `npm install` to install dependencies
1. Run `npm run dev` to start the React app

### Node Server

1. cd into the `node_server` directory and run `npm install` to install dependencies
1. Run `npm run dev` to start the Node server

### Rust Server

1. cd into the `rust_server` directory
1. Run `cargo run` to start the Rust server
