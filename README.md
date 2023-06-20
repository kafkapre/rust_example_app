ukaz jak nastavit clippy v Golandu



### Prerequisites

#### Postgres
- `docker run --name rust-example-app-db -p 5431:5432 --restart=unless-stopped -e POSTGRES_HOST_AUTH_METHOD=trust -d postgres:15.0`

#### Jaeger
- `docker run -d -p 6831:6831/udp -p 6832:6832/udp -p 16686:16686 jaegertracing/all-in-one:latest`

#### Prometheus
- download Prometheus from [https://prometheus.io/download/]
- copy prometheus.yml into dir where prometheus app is downloaded
- run Prometheus app `./prometheus`
- `localhost:9090`
- search e.g. for `response_time_bucket`

### Cargo interesting tools
- cargo clippy - check for common mistakes and opportunities for improvement
- cargo fmt - format your code automatically
- cargo tree - display your dependency tree so you can see where you can try to reduce compile times
- cargo flamegraph - profile your code and show the result as a flamegraph in a browser
- mold (Linux only) - a much faster alternative to the default ld linker
