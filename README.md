# golem_async

### Build & Run

1. `docker-compose -f docker-compose-postgres.yaml up`
2. `cd golem_async`
3. `cargo component build --release`
4. `golem-cli component add -c golem_async target/wasm32-wasi/release/golem_async.wasm`
5. `golem-cli worker add -c golem_async -w golem_async_1`
6. `golem-cli worker invoke-and-await -c golem_async -w golem_async_1 -f 'golem:component/api.{start}'`

### Delete Worker

- `golem-cli worker delete -c golem_async -w golem_async_1`

### Rebuild & Redeploy

- `cargo component build --release && golem-cli component add -y -c golem_async target/wasm32-wasi/release/golem_async.wasm && golem-cli component redeploy -y -c golem_async`
