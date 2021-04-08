# Rust TCP Server/Client
This project has an example of a simple TCP server and client. We've included the dockerfiles to be able to create 2 containers.

## Creating the containers
To create the tcp_server do:

```
$ cd tcp_server
$ DOCKER_BUILDKIT=1 docker build -t tcp_server .
```

To create the tcp_client do:

```
$ cd tcp_client
$ DOCKER_BUILDKIT=1 docker build -t tcp_client .
```

## Running the containers
To run the tcp_server:
```
$ docker run -p 3333:3333 --name rust_server tcp_server
```
To run the tcp_client:
```
$ docker run --link rust_server tcp_client rust_server
```

## Expected output
If everything goes right you should see the following in the tcp_server terminal:
```
$ docker run -p 3333:3333 --name rust_server tcp_server
Server listening on port 3333
New connection: 172.17.0.3:60974
```
And this in the tcp_client terminal:
```
$ docker run --link rust_server tcp_client rust_server
Successfully connected to server in port 3333
Sent Hello, awaiting reply...
Reply is ok!
Terminated.
```