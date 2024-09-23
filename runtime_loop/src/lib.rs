mod bindings;

use crate::bindings::exports::golem::component_runtime_loop::api_runtime_loop::*;
use crate::bindings::golem::component_golem_async_stub::stub_golem_async::*;
use golem_rust::bindings::wasi;

struct Component;

impl Guest for Component {
    fn start_loop() {
        let golem_async_worker = ApiGolemAsync::new(&GolemRpcUri {
            value: "worker://golem_async/golem_async_1".to_owned()
        });
        println!("Loop started!");
        loop {
            wasi::clocks::monotonic_clock::subscribe_duration(2_000_000_000).block();
            println!("calling 'run_all_tasks()' ...");
            golem_async_worker.blocking_run_all_tasks();
            println!("'run_all_tasks()' called");
        }
    }
}

bindings::export!(Component with_types_in bindings);
