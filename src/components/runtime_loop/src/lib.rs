mod bindings;

use crate::bindings::exports::golem::component_runtime_loop::api_runtime_loop::*;
use crate::bindings::golem::component_counter_stub::stub_counter::*;
use golem_rust::bindings::wasi::clocks::monotonic_clock::{self, Duration};
use std::env;

struct Component;

const MILLISECOND: Duration =  1_000_000;
const WAIT_TIME: Duration = 2_000 * MILLISECOND;

impl Guest for Component {
    fn start_loop() {
        let component_id = env::var("COUNTER_COMPONENT_ID").expect("COUNTER_COMPONENT_ID not set");
        let counter_worker = ApiCounter::new(&GolemRpcUri {
            value: format!("urn:worker:{component_id}/counter_1")
        });
        println!("Loop starting!");
        loop {
            monotonic_clock::subscribe_duration(WAIT_TIME).block();
            println!("calling 'run_all_tasks()' ...");
            counter_worker.blocking_run_all_tasks();
            println!("'run_all_tasks()' called");
        }
    }
}

bindings::export!(Component with_types_in bindings);
