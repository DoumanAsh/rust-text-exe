# rust test exercise

## Task

Write a simple non-blocking API in rust that allows a client to request a piece of information from a server. Write an example client and server to demonstrate its functionality. The client asks for a number x: u32 of addresses, and the server responds with a list of x random socket addresses:

```rust
use std::net::SocketAddr;
struct Response {
   addr: Vec<SocketAddr>, // Length = x, each SocketAddr is a random IP
                          // and a random port.
}
```

```rust
x == 0 means “exit”.
```

For example:

* The client connects to the server.
* Ask user (CLI) for `x` -> user enters 3, request is sent to server
* Ask user (CLI) for `x` -> user enters 5, request is sent to server
* The response with 3 addresses returns and is displayed.
* The response with 5 addresses returns and is displayed.
* Ask user (CLI) for `x` -> user enters 0
* exit client gracefully

The server must be non-blocking (sockets, listeners everything) while the choice of client is entirely upto you - both blocking and non-blocking are acceptable. You can write them as two separate programs or in the same program (use separate modules if you want). The client code should be able to connect to the server at-least on the same LAN in simple setups. You are welcome to use mio rust crate for non-blocking code and rand crate for getting random values of appropriate types.

The given struct Response represents the wire format of what server sends the client. Use an appropriate way to send it over the stream.

Client should know where to connect to the server via some discovery mechanism. So come up with a simple (server) discovery mechanism. Explain it in a few sentences either as an inline documentation or in some associated README file of what you have used/chosen.

## Usage

For now both server and client uses hard coded address: `localhost:666`
Just start server and then client, and you're free to go.
