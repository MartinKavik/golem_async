# Golem Async

### Start Golem

- `docker-compose -f docker-compose-postgres.yaml up`

### Build & Add Components

#### Debug
1. `makers build-flow`
2. `golem-cli component add -y -c counter target/wasm32-wasi/debug/counter_composed.wasm`
3. `golem-cli component add -y -c runtime_loop target/wasm32-wasi/debug/runtime_loop_composed.wasm`

#### Release
1. `makers release-build-flow`
2. `golem-cli component add -y -c counter target/wasm32-wasi/release/counter_composed.wasm`
3. `golem-cli component add -y -c runtime_loop target/wasm32-wasi/release/runtime_loop_composed.wasm`

### Add Workers

1. `golem-cli component get -c runtime_loop` => `urn:component:{COMPONENT_ID}`
2. `golem-cli worker add -c counter -w counter_1 -e RUNTIME_LOOP_COMPONENT_ID={COMPONENT_ID}`
3. `golem-cli component get -c counter` => `urn:component:{COMPONENT_ID}`
4. `golem-cli worker add -c runtime_loop -w runtime_loop_1 -e counter_COMPONENT_ID={COMPONENT_ID}`

### Connect to Workers

1. `golem-cli worker connect -c counter -w counter_1`
2. `golem-cli worker connect -c runtime_loop -w runtime_loop_1`

### Call Workers

1. `golem-cli worker invoke-and-await -c counter -w counter_1 -f 'golem:component-golem-async/api-golem-async.{add}' -a '2'`
2. `golem-cli worker invoke-and-await -c counter -w counter_1 -f 'golem:component-golem-async/api-golem-async.{get}'`

---

### Rebuild Components & Redeploy Workers

#### Debug

- `makers build-flow && golem-cli component add -y -c counter target/wasm32-wasi/debug/counter_composed.wasm && golem-cli component redeploy -y -c counter && golem-cli component add -y -c runtime_loop target/wasm32-wasi/debug/runtime_loop_composed.wasm && golem-cli component redeploy -y -c runtime_loop`

#### Release

- `makers release-build-flow && golem-cli component add -y -c counter target/wasm32-wasi/release/counter_composed.wasm && golem-cli component redeploy -y -c counter && golem-cli component add -y -c runtime_loop target/wasm32-wasi/release/runtime_loop_composed.wasm && golem-cli component redeploy -y -c runtime_loop`

### How to join the components

1. `golem-cli stubgen initialize-workspace --targets src/components/counter --targets src/components/runtime_loop --callers src/components/counter --callers src/components/runtime_loop`
