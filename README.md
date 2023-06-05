# near-http-spin
Simple HTTP application built on-top of the Spin Framework, written in Rust. The application is a web server that acts as an HTTP trigger by listening for incoming requests, routing them to a specified Component, and returning an HTTP response. 

The routes defined in the `Application Manifest` compile down to binary Wasm modules i.e. `Components`. In this case, the application listens for incoming requests and routes the request to a component in charge of executing specific calls to NEAR blockchain JSON-RPC API endpoints.

### What is Spin?
Spin is a framework for building and running event-driven microservice applications with WebAssembly (Wasm) components.

Spin uses Wasm because it is sandboxed, portable, and fast. Millisecond cold start times mean no need to keep applications “warm”.
