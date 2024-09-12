# Shahi: Redis-like storage written in rust
Shahi is a redis-like storage software written in rust. it supports basic functionalities such as defining key-values.
## Usage
```bash
$ ./shahi 7878
```
## Example
```bash
$nc 127.0.0.1 7878
set key1 "value"

Ok

$nc 127.0.0.1 7878
get key1

value
```
