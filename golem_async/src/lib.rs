mod bindings;

use crate::bindings::exports::golem::component::api::*;

struct Component;

impl Guest for Component {
    fn start() -> String {
        "Hello from golem_async!".to_owned()
    }
}

bindings::export!(Component with_types_in bindings);
