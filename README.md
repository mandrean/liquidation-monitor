# liquidation-monitor

### Challenge
Create a web service in Rust that monitors the bLUNA liquidation prices of collateralized loans on Anchor in real-time, and presents them (at $0.10 increments) over a REST API. 

### Solution
The service has basically 3 parts:

1. A WebSocket client listening to the stream of events on Terra Observer
2. A shared state/cache of the borrowers and their collateralized loans on Anchor
3. A web server exposing all the loan data over a REST API

In order to better simulate real-world circumstances, I also wrote a simple crawler that pulls out all existing loans from Anchor's GraphQL API.
This seed data is loaded once when the service starts, and from that point on, it's modified according to the incoming events (`borrow_stable`, `repay_stable`, `deposit_collateral`, `withdraw_collateral`).

This seed data consists of roughly 25,000 loans (before filtering out ones with 0 bLUNA collateral).

### Usage
```sh
# start the monitor (or compile with --release for maximum performance)
$ cargo run .

# for logging, set the RUST_LOG env var
$ RUST_LOG=liquidation_monitor=debug cargo run .

# or even more verbose
$ RUST_LOG=liquidation_monitor=trace cargo run .

# curl the list of borrowers
$ curl 127.0.0.1:8080/api/borrowers | jq

# curl the bLUNA liquidation prices if bETH goes to $1,500
$ curl 127.0.0.1:8080/api/liqs?beth_price\=1500000000 | jq

# curl the bLUNA liquidation prices if bETH goes to $2,000
$ curl 127.0.0.1:8080/api/liqs?beth_price\=2000000000 | jq
```

### Concurrency
The service is written with concurrency in mind. It uses `async` extensively, including a newer version of `rocket` with `async` support.
The shared state is accessed using Tokio's _fair_ `RwLock` instead of `Mutex`, in order to allow for _single-writer/many-readers_.

### Performance
The underlying data structure of the shared state is a binary tree-map, which allows for good overall performance when fetching entries or inserting new ones.
Since this solution also takes bETH collateral/bETH price into account, we can't just cache all loans to be liquidated using the liquidation price of bLUNA as cache key.

To work around this, we use a Timed LRU cache of the serialized output (using the provided bETH price as cache key).
Some basic load-testing/benchmarking shows that the service can handle 100s of concurrent requests at the same time with less than 1s average response times on my old laptop when dumping the full loan liquidations data

### Note about Columbus-5
The rollout of Columbus-5 introduced some breaking changes to the APIs. I tried to fix most of the stuff I could find, but there might still be some newly introduced ones in there.
One such bug would be the cache-fallback using `MantleClient::query_loan()` for example.

### Further improvements
- Better use of borrowing & lifetimes. Due to time constraints, it was quicker to move/clone data instead.
- Profiling using `flame` to find out if there are any glaring performance bottlenecks to fix.
- Gzip Compression. Rocket doesn't have built in support for this right now. Putting the service behind for example `nginx` could cut down the amount of transferred data by a lot.
- Lock-free data structures. It would be interesting try STMs or skip list-backed maps ("skip maps") and compare performance under load.
- Ranges & Pagination. Right now, the API just dumps out all data (~700 kB). In production, you would probably also want the ability to query only ranges of liquidation prices, and perhaps to paginate the response.
- Persistent state. In production you would perhaps want to sync the state to Cassandra or Redis.
- Better use of traits. Less leaky abstractions.
- More tests. Due to time constraints, there wasn't enough time to properly test everything.

### Dependencies
- tokio - Async runtime & data structures
- tungstenite - WebSocket client
- cynic - GraphQL client
- surf - HTTP client
- rocket - HTTP server
- rust_decimal - Decimal type without round-off errors
- cached - Timed LRU cache
- serde - JSON serialization/deserialization
- tracing - Logging/tracing
- anyhow - Error handling
