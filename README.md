# Golem Async

This is a [Golem](https://www.golem.cloud/) testing example. It consists two _Components_ and two associated _Workers_ talking to each other. 

The `counter` component (`counter_1` worker) increments its value or returns its current value on an API call. It _reactively_ (see the crate [futures-signals](https://crates.io/crates/futures-signals)) writes the latest counter value into a file. The _Futures_ used in the `counter` component to support reactivity are driven by a simpler async runtime from the [futures-executor](https://crates.io/crates/futures-executor) crate.

The `runtime_loop` component - respectively the `runtime_loop_1` worker - calls `counter_1` API in a loop to drive pending futures. The loop iterates every 2 seconds currently.

There are two ways to run the example - with the released Golem tools and Docker files or with cloned [Golem repo](https://github.com/golemcloud/golem). You have to clone both Golem repo and this repo into the same parent directory to not brake predefine script paths.

The code was mostly tested on `Kubuntu 24.04` and `Rust 1.82.0` with a specific forked Golem [branch](https://github.com/MartinKavik/golem/tree/initial_file_system) so expect some potential issues if you try to run it. Golem currently needs some services like Redis to run and it doesn't fully support Windows. Consult their docs for more info.

_Note_: `makers` is the alternative executable of the script runner [cargo-make](https://crates.io/crates/cargo-make).

## With local `golem-cli` and Golem services (without Docker)

### Start Golem

1. In the cloned Golem root dir: `makers run`

### Build & Add Components

#### Debug
1. `makers build-flow`
2. `makers golem-cli component add -y -c counter target/wasm32-wasip1/debug/counter_composed.wasm`
3. `makers golem-cli component add -y -c runtime_loop target/wasm32-wasip1/debug/runtime_loop_composed.wasm`

#### Release
1. `makers release-build-flow`
2. `makers golem-cli component add -y -c counter target/wasm32-wasip1/release/counter_composed.wasm`
3. `makers golem-cli component add -y -c runtime_loop target/wasm32-wasip1/release/runtime_loop_composed.wasm`

### Add Workers

1. `LOCAL_GOLEM_CLI="../golem/target/debug/golem-cli"`
2. `RUNTIME_LOOP_COMPONENT_ID=$($LOCAL_GOLEM_CLI component get -c runtime_loop --format json | jq '.componentUrn | ltrimstr("urn:component:")' -r)`
3. `makers golem-cli worker add -c counter -w counter_1 -e RUNTIME_LOOP_COMPONENT_ID=$RUNTIME_LOOP_COMPONENT_ID`
4. `COUNTER_COMPONENT_ID=$($LOCAL_GOLEM_CLI component get -c counter --format json | jq '.componentUrn | ltrimstr("urn:component:")' -r)`
5. `makers golem-cli worker add -c runtime_loop -w runtime_loop_1 -e COUNTER_COMPONENT_ID=$COUNTER_COMPONENT_ID`

### Connect to Workers

1. `makers golem-cli worker connect -c counter -w counter_1`
2. `makers golem-cli worker connect -c runtime_loop -w runtime_loop_1`

### Call Workers

1. `makers golem-cli worker invoke-and-await -c counter -w counter_1 -f 'golem:component-counter/api-counter.{add}' -a '2'`
2. `makers golem-cli worker invoke-and-await -c counter -w counter_1 -f 'golem:component-counter/api-counter.{get}'`

### Call Worker Files API

1. Add workers + set env vars (see `Add Workers` above)
2. `curl "http://localhost:9881/v1/components/$COUNTER_COMPONENT_ID/workers/counter_1/files?recursive=true&path=images" -s | jq .`

---

### Rebuild Components & Redeploy Workers

### Full Rebuild & Redeploy:

```bash
makers golem-cli worker delete -c counter -w counter_1 
makers golem-cli worker delete -c runtime_loop -w runtime_loop_1 
makers build-flow
makers golem-cli component add -y -c counter target/wasm32-wasip1/debug/counter_composed.wasm
makers golem-cli component add -y -c runtime_loop target/wasm32-wasip1/debug/runtime_loop_composed.wasm
LOCAL_GOLEM_CLI="../golem/target/debug/golem-cli"
RUNTIME_LOOP_COMPONENT_ID=$($LOCAL_GOLEM_CLI component get -c runtime_loop --format json | jq '.componentUrn | ltrimstr("urn:component:")' -r)
makers golem-cli worker add -c counter -w counter_1 -e RUNTIME_LOOP_COMPONENT_ID=$RUNTIME_LOOP_COMPONENT_ID
COUNTER_COMPONENT_ID=$($LOCAL_GOLEM_CLI component get -c counter --format json | jq '.componentUrn | ltrimstr("urn:component:")' -r)
makers golem-cli worker add -c runtime_loop -w runtime_loop_1 -e COUNTER_COMPONENT_ID=$COUNTER_COMPONENT_ID
```

#### Debug

- `makers build-flow && makers golem-cli component add -y -c counter target/wasm32-wasip1/debug/counter_composed.wasm && makers golem-cli component redeploy -y -c counter && makers golem-cli component add -y -c runtime_loop target/wasm32-wasip1/debug/runtime_loop_composed.wasm && makers golem-cli component redeploy -y -c runtime_loop`

#### Release

- `makers release-build-flow && makers golem-cli component add -y -c counter target/wasm32-wasip1/release/counter_composed.wasm && makers golem-cli component redeploy -y -c counter && makers golem-cli component add -y -c runtime_loop target/wasm32-wasip1/release/runtime_loop_composed.wasm && makers golem-cli component redeploy -y -c runtime_loop`

### How to join the components

1. `makers golem-cli stubgen initialize-workspace --targets src/components/counter --targets src/components/runtime_loop --callers src/components/counter --callers src/components/runtime_loop`

---

## With official `golem-cli` and `docker-compose`

### Start Golem

- `docker-compose -f docker-compose-postgres.yaml up`

### Build & Add Components

#### Debug
1. `makers -e OFFICIAL=1 build-flow`
2. `golem-cli component add -y -c counter target/wasm32-wasip1/debug/counter_composed.wasm`
3. `golem-cli component add -y -c runtime_loop target/wasm32-wasip1/debug/runtime_loop_composed.wasm`

#### Release
1. `makers -e OFFICIAL=1 release-build-flow`
2. `golem-cli component add -y -c counter target/wasm32-wasip1/release/counter_composed.wasm`
3. `golem-cli component add -y -c runtime_loop target/wasm32-wasip1/release/runtime_loop_composed.wasm`

### Add Workers

1. `RUNTIME_LOOP_COMPONENT_ID=$(golem-cli component get -c runtime_loop --format json | jq '.componentUrn | ltrimstr("urn:component:")' -r)`
2. `golem-cli worker add -c counter -w counter_1 -e RUNTIME_LOOP_COMPONENT_ID=$RUNTIME_LOOP_COMPONENT_ID`
3. `COUNTER_COMPONENT_ID=$(golem-cli component get -c counter --format json | jq '.componentUrn | ltrimstr("urn:component:")' -r)`
4. `golem-cli worker add -c runtime_loop -w runtime_loop_1 -e COUNTER_COMPONENT_ID=$COUNTER_COMPONENT_ID`

### Connect to Workers

1. `golem-cli worker connect -c counter -w counter_1`
2. `golem-cli worker connect -c runtime_loop -w runtime_loop_1`

### Call Workers

1. `golem-cli worker invoke-and-await -c counter -w counter_1 -f 'golem:component-counter/api-counter.{add}' -a '2'`
2. `golem-cli worker invoke-and-await -c counter -w counter_1 -f 'golem:component-counter/api-counter.{get}'`

---

### Rebuild Components & Redeploy Workers

#### Debug

- `makers -e OFFICIAL=1 build-flow && golem-cli component add -y -c counter target/wasm32-wasip1/debug/counter_composed.wasm && golem-cli component redeploy -y -c counter && golem-cli component add -y -c runtime_loop target/wasm32-wasip1/debug/runtime_loop_composed.wasm && golem-cli component redeploy -y -c runtime_loop`

#### Release

- `makers -e OFFICIAL=1 release-build-flow && golem-cli component add -y -c counter target/wasm32-wasip1/release/counter_composed.wasm && golem-cli component redeploy -y -c counter && golem-cli component add -y -c runtime_loop target/wasm32-wasip1/release/runtime_loop_composed.wasm && golem-cli component redeploy -y -c runtime_loop`

### How to join the components

1. `golem-cli stubgen initialize-workspace --targets src/components/counter --targets src/components/runtime_loop --callers src/components/counter --callers src/components/runtime_loop`
