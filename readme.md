# Rust framework benchmarks

Benchmarks of most widely used [rust](https://rust-lang.org) web frameworks.

## Frameworks included
**[hyper](https://hyper.rs)**

**[axum](https://github.com/tokio-rs/axum)**

**[actix-web](https://actix.rs)**

**[ntex](https://github.com/ntex-rs/ntex)**

**[warp](https://github.com/seanmonstar/warp)**

**[tide](https://github.com/http-rs/tide)**


#
### Thread: 1 | Concurrency: 10 | Duration: 20 secs
#

|   Name   |   Req/sec   | Avg Latency | Max Latency |   # Requests  |
|:--------:|:-----------:|:-----------:|:-----------:|:-------------:|
|   ntex   |   228,941.  |   0.032 ms  |   2.89 ms   |   4,601,755   |
|   actix  |   223,726   |   0.033 ms  |   3.77 ms   |   4,496,881   |
|   hyper  |   205,325   |   0.036 ms  |   3.02 ms   |   4,126,939   |
|   axum   |   203,453   |   0.037 ms  |   3.66 ms   |   4,089,273   |
|   warp   |   200,585   |   0.042 ms  |   10.00 ms  |   4,012,148   |
|   tide   |   127,537   |   0.069 ms  |   2.38 ms   |   2,563,488   |

#
### Thread: 1 | Concurrency: 100 | Duration: 20 secs
#

|   **Name**   |   Req/sec   | Avg Latency | Max Latency |  # Requests |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|
|   **hyper**  |   237,480   |   0.228 ms  |   2.82 ms   |  4,773,611  |
|   **warp**   |   232,012   |   0.233 ms  |   1.94 ms   |  4,663,878  |
|   **axum**   |   229,416   |   0.238 ms  |   3.02 ms   |  4,611,457  |
|   **actix**  |   206,339   |   0.260 ms  |   2.32 ms   |  4,147,603  |
|   **ntex**   |   201,824   |   0.266 ms  |   2.88 ms   |  4,056,874  |
|   **tide**   |    51,483   |   2.100 ms  |   27.15 ms  |  1,034,990  |

#
### Thread: 1 | Concurrency: 1000 | Duration: 30 secs
#

|   **Name**   |   Req/sec   | Avg Latency | Max Latency |  # Requests |
|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|
|   **hyper**  |   163,132   |   3.100 ms  |   17.20 ms   |  4,905,895  |
|   **warp**   |   162,803   |   3.100 ms  |   17.80 ms   |  4,898,235  |
|   **axum**   |   157,119   |   3.220 ms  |   16.12 ms   |  4,725,407  |
|   **ntex**   |   132,546   |   3.800 ms  |   12.15 ms   |  3,983,054  |
|   **actix**  |   130,516   |   3.870 ms  |   12.61 ms   |  3,917,247  |
|   **tide**   |    42,289   |   27.17 ms  |   182.9 ms   |  1,269,557  |

## Benchmarking tool
The benchmarks have been performed using [wrk](https://github.com/wg/wrk), locally. 

## Try it yourself
To run the code please follow the steps - 

1. Download the repository as a zip, or clone/fork it.
2. `cd rust-framework-benchmarks`
3. `cargo build --release`
4. Open multiple terminals and start each server (if you want to run all simultaneously). 
Eg, `./target/release/actix` on one, `./target/release/hyper` on another and so on.
5. Run batch tests - `sh ./start-test.sh`

All the output will be stored in `perf/*`

## Machine used
M1 Max MacBook Pro 2021 - 64GB ram, 10 CPU cores and 32 GPU cores

## Suggestions and changes
All the suggestions, code changes or additions of another web framework is appreciated. I'd like to keep the code as close as a real world scenario, instead of optimising it to the metal.
