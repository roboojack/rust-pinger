# Latency

## About
Latency is a simple rust command line app that pings an http service and prints the:

 - number of bytes retrieved
 - min response time
 - max response time
 - mean response time

## Usage


`cargo run {URL} {PingTimes}`

eg: `cargo run https://yahoo.com 3`