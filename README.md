# ppbench: benchmark of pingpong channel messaging

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

## Result

- `count = 10000000`
- Tested on
  - Rust 1.26.2
  - CentOS 6.9
  - Intel(R) Xeon(R) CPU E5-2690 v2 @ 3.00GHz

| Kind       | Elapsed time [secs] |
| ---------- | ------------------- |
| Ping       | 1.782312            |
| PingPong   | 178.752675          |
| CbPing     | 8.397810            |
| CbPingPong | 14.992134           |
