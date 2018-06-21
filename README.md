# ppbench: benchmark of pingpong channel messageing

## Usage

```
Benchmark of pingpong channel messaging.

Usage:
  ppbench [option] <kind> <count>
  ppbench (-h | --help)

Arguments:
  kind   Kinds of pingpong:
           Ping        One-way messaging (request).
           PingPong    Two-way messaging (request and response).
           CbPing      crossbeam-channel version of Ping.
           CbPingPong  crossbeam-channel version of PingPong.
  count  # of pingpong.

Options:
  -h --help      Show this message.
```
