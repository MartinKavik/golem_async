# golem_async

### Build & Run

1. `docker-compose -f docker-compose-postgres.yaml up`
2. `cargo component build -r`
4. `golem-cli component add -c golem_async target/wasm32-wasi/release/golem_async.wasm`
5. `golem-cli worker add -c golem_async -w golem_async_1`
8. `golem-cli component add -c runtime_loop target/wasm32-wasi/release/runtime_loop.wasm`
9. `golem-cli worker add -c runtime_loop -w runtime_loop_1`
10. `golem-cli worker invoke-and-await -c golem_async -w golem_async_1 -f 'golem:component/api.{add}' -a '2'`
11. `golem-cli worker invoke-and-await -c golem_async -w golem_async_1 -f 'golem:component/api.{get}'`

### Rebuild & Redeploy

1. `cargo component build -r && golem-cli component add -y -c golem_async target/wasm32-wasi/release/golem_async.wasm && golem-cli component redeploy -y -c golem_async && golem-cli component add -y -c runtime_loop target/wasm32-wasi/release/runtime_loop.wasm && golem-cli component redeploy -y -c runtime_loop`

### Connect to Workers

1. `golem-cli worker connect -c golem_async -w golem_async_1`
2. `golem-cli worker connect -c runtime_loop -w runtime_loop_1`
