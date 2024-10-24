#![allow(warnings)]
use golem_wasm_rpc::*;
#[allow(dead_code)]
mod bindings;
pub struct ApiRuntimeLoop {
    rpc: WasmRpc,
}
impl ApiRuntimeLoop {}
struct Component;
impl crate::bindings::exports::golem::component_runtime_loop_stub::stub_runtime_loop::Guest
for Component {
    type ApiRuntimeLoop = crate::ApiRuntimeLoop;
}
impl crate::bindings::exports::golem::component_runtime_loop_stub::stub_runtime_loop::GuestApiRuntimeLoop
for ApiRuntimeLoop {
    fn new(location: crate::bindings::golem::rpc::types::Uri) -> Self {
        let location = golem_wasm_rpc::Uri {
            value: location.value,
        };
        Self {
            rpc: WasmRpc::new(&location),
        }
    }
    fn blocking_start_loop(&self) -> () {
        let result = self
            .rpc
            .invoke_and_await(
                "golem:component-runtime-loop/api-runtime-loop.{start-loop}",
                &[],
            )
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}",
                    "golem:component-runtime-loop/api-runtime-loop.{start-loop}"
                ),
            );
        ()
    }
    fn start_loop(&self) -> () {
        let result = self
            .rpc
            .invoke("golem:component-runtime-loop/api-runtime-loop.{start-loop}", &[])
            .expect(
                &format!(
                    "Failed to invoke remote {}",
                    "golem:component-runtime-loop/api-runtime-loop.{start-loop}"
                ),
            );
        ()
    }
}
bindings::export!(Component with_types_in bindings);
