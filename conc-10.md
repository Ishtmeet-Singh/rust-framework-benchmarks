|   Name   |   Req/sec   | Avg Latency | Max Latency |   # Requests  |
|:--------:|:-----------:|:-----------:|:-----------:|:-------------:|
|   ntex   |   228,941.  |   0.032 ms  |   2.89 ms   |   4,601,755   |
|   actix  |   223,726   |   0.033 ms  |   3.77 ms   |   4,496,881   |
|   hyper  |   205,325   |   0.036 ms  |   3.02 ms   |   4,126,939   |
|   axum   |   203,453   |   0.037 ms  |   3.66 ms   |   4,089,273   |
|   warp   |   200,585   |   0.042 ms  |   10.00 ms  |   4,012,148   |
|   tide   |   127,537   |   0.069 ms  |   2.38 ms   |   2,563,488   |