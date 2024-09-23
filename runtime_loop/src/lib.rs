mod bindings;

use crate::bindings::exports::golem::component::api::*;
use golem_rust::bindings::wasi;

struct Component;

impl Guest for Component {
    fn start_loop() {
        println!("Loop started!");
        loop {
            wasi::clocks::monotonic_clock::subscribe_duration(2_000_000_000).block();
            println!("loop iteration!");
        }
    }
}

bindings::export!(Component with_types_in bindings);
