mod bindings;

use crate::bindings::exports::golem::component_counter::api_counter::*;
use futures_signals::signal::{Mutable, SignalExt};
use serde::Deserialize;
use std::path::{Path, PathBuf};
use std::fs;
use std::sync::LazyLock;

fn read_only_file_path(relative_file_path: impl AsRef<Path>) -> PathBuf {
    Path::new("/system/component/current/files").join(relative_file_path)
}

#[derive(Deserialize)]
struct State {
    total: Mutable<u64>,
}

static STATE: LazyLock<State> = LazyLock::new(|| {
    let state_json_path = read_only_file_path("default_counter.json");
    let state_json = fs::read_to_string(&state_json_path)
        .map_err(|error| {
            eprintln!("failed to read 'default_counter.json': {error:#}")
        })
        .unwrap_or_else(|_| String::from("{ \"total\": 22 }"));

    let state: State = serde_json::from_str(&state_json)
        .expect("failed to parse JSON from 'default_counter.json'");

    task::spawn(state.total.signal().for_each(move |total| {
        println!("--------");

        match fs::read_to_string(read_only_file_path(&state_json_path)){
            Ok(content) => println!("Content of 'default_counter.json': {content}"),
            Err(error) => eprintln!("Failed to read 'default_counter.json': {error:#}")
        }

        println!("Total changed to: {total}!");
        let last_value_path = Path::new("last_value.txt");

        let previous_value = fs::read_to_string(last_value_path)
            .expect("failed to read 'last_value.txt'");

        println!("Previous value read from file: {}", previous_value.trim());

        fs::write(last_value_path, total.to_string())
            .expect("failed to write to 'last_value.txt'");

        async {}
    }));
    state
});

struct Component;

impl Guest for Component {
    fn add(value: u64) {
        *STATE.total.lock_mut() += value;
    }

    fn get() -> u64 {
        let total = STATE.total.get();
        total
    }

    fn run_all_tasks() {
        task::run_all();
    }
}

bindings::export!(Component with_types_in bindings);

mod task {
    use futures_executor::{LocalPool, LocalSpawner};
    use futures_task::LocalSpawn;
    use std::cell::RefCell;
    use std::future::Future;
    use std::env;

    use crate::bindings::golem::component_runtime_loop_stub::stub_runtime_loop::*;

    thread_local! {
        static TASK_POOL: RefCell<LocalPool> = Default::default();
        static TASK_SPAWNER: LocalSpawner = TASK_POOL.with_borrow(|pool| {
            let component_id = env::var("RUNTIME_LOOP_COMPONENT_ID").expect("RUNTIME_LOOP_COMPONENT_ID not set");
            let runtime_loop_worker = ApiRuntimeLoop::new(&GolemRpcUri {
                value: format!("urn:worker:{component_id}/runtime_loop_1")
            });
            println!("Calling loop worker to start the loop...");
            runtime_loop_worker.start_loop();
            pool.spawner()
        });
    }

    pub fn spawn(future: impl Future<Output = ()> + 'static) {
        TASK_SPAWNER.with(move |spawner| spawner.spawn_local_obj(Box::pin(future).into()).unwrap())
    }

    pub fn run_all() {
        TASK_POOL.with_borrow_mut(|pool| pool.run_until_stalled())
    }
}
