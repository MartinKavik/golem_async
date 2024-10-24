mod bindings;

use crate::bindings::exports::golem::component_counter::api_counter::*;
use futures_signals::signal::{Mutable, SignalExt};
use std::sync::LazyLock;

#[derive(Default)]
struct State {
    total: Mutable<u64>,
}

static STATE: LazyLock<State> = LazyLock::new(|| {
    let state = State::default();
    task::spawn(state.total.signal().for_each(|total| {
        println!("total changed: {total}!");
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
