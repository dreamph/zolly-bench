# zolly-bench

Zolly API Gateway Benchmark

### Benchmark

- MacBook Pro 2023
- Chip Apple M2 Max
- Memory 64 GB

### Setup Tools
```shell
make setup
```

### Start Server
```shell
make build-server run-server
```

### Start KrakenD
```shell
cd krakend && make run
```

### Start Zolly
```shell
cd zolly && make run
```

### LoadTest without API Gateway

```shell
wrk -t12 -c100 -d60s http://127.0.0.1:8000/v1/hello
Running 1m test @ http://127.0.0.1:8000/v1/hello
  12 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     1.06ms    5.16ms 229.90ms   98.87%
    Req/Sec    11.76k     2.72k   68.56k    92.35%
  8423482 requests in 1.00m, 1.03GB read
Requests/sec: 140187.71
Transfer/sec:     17.51MB
```

### LoadTest Zolly API Gateway

```shell
wrk -t12 -c100 -d60s http://127.0.0.1:8070/v1/hello
Running 1m test @ http://127.0.0.1:8070/v1/hello
  12 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     4.63ms   16.56ms 376.43ms   95.09%
    Req/Sec     6.40k     2.90k   34.92k    82.16%
  4579541 requests in 1.00m, 572.13MB read
  Socket errors: connect 0, read 0, write 0, timeout 1
Requests/sec:  76203.34
Transfer/sec:      9.52MB
```

### LoadTest KrakenD API Gateway

```shell
wrk -t12 -c100 -d60s http://127.0.0.1:8090/v1/hello
Running 1m test @ http://127.0.0.1:8090/v1/hello
  12 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     2.02ms    2.01ms  70.55ms   94.56%
    Req/Sec     4.47k   416.06     6.27k    70.93%
  3200251 requests in 1.00m, 717.22MB read
Requests/sec:  53322.81
Transfer/sec:     11.95MB
```