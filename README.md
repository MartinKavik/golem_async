# Golem Async

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

1. `RUNTIME_LOOP_COMPONENT_ID=$(golem-cli component
 get -c runtime_loop --format json | jq '.componentUrn | ltrimstr("urn:component:"
)' -r)`
2. `golem-cli worker add -c counter -w counter_1 -e RUNTIME_LOOP_COMPONENT_ID=$RUNTIME_LOOP_COMPONENT_ID`
3. `COUNTER_COMPONENT_ID=$(golem-cli component
 get -c counter --format json | jq '.componentUrn | ltrimstr("urn:component:"
)' -r)`
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

---

## With local `golem-cli` and Golem services (without Docker)

### Start Golem

In cloned https://github.com/golemcloud/golem:
- `makers run`

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
2. `RUNTIME_LOOP_COMPONENT_ID=$($LOCAL_GOLEM_CLI component
 get -c runtime_loop --format json | jq '.componentUrn | ltrimstr("urn:component:"
)' -r)`
3. `makers golem-cli worker add -c counter -w counter_1 -e RUNTIME_LOOP_COMPONENT_ID=$RUNTIME_LOOP_COMPONENT_ID`
4. `COUNTER_COMPONENT_ID=$($LOCAL_GOLEM_CLI component
 get -c counter --format json | jq '.componentUrn | ltrimstr("urn:component:"
)' -r)`
5. `makers golem-cli worker add -c runtime_loop -w runtime_loop_1 -e COUNTER_COMPONENT_ID=$COUNTER_COMPONENT_ID`

### Connect to Workers

1. `makers golem-cli worker connect -c counter -w counter_1`
2. `makers golem-cli worker connect -c runtime_loop -w runtime_loop_1`

### Call Workers

1. `makers golem-cli worker invoke-and-await -c counter -w counter_1 -f 'golem:component-counter/api-counter.{add}' -a '2'`
2. `makers golem-cli worker invoke-and-await -c counter -w counter_1 -f 'golem:component-counter/api-counter.{get}'`

---

### Rebuild Components & Redeploy Workers

#### Debug

- `makers build-flow && makers golem-cli component add -y -c counter target/wasm32-wasip1/debug/counter_composed.wasm && makers golem-cli component redeploy -y -c counter && makers golem-cli component add -y -c runtime_loop target/wasm32-wasip1/debug/runtime_loop_composed.wasm && makers golem-cli component redeploy -y -c runtime_loop`

#### Release

- `makers release-build-flow && makers golem-cli component add -y -c counter target/wasm32-wasip1/release/counter_composed.wasm && makers golem-cli component redeploy -y -c counter && makers golem-cli component add -y -c runtime_loop target/wasm32-wasip1/release/runtime_loop_composed.wasm && makers golem-cli component redeploy -y -c runtime_loop`

### How to join the components

1. `makers golem-cli stubgen initialize-workspace --targets src/components/counter --targets src/components/runtime_loop --callers src/components/counter --callers src/components/runtime_loop`
