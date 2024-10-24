#![allow(warnings)]
use golem_wasm_rpc::*;
#[allow(dead_code)]
mod bindings;
pub struct ApiCounter {
    rpc: WasmRpc,
}
impl ApiCounter {}
pub struct FutureGetResult {
    pub future_invoke_result: FutureInvokeResult,
}
struct Component;
impl crate::bindings::exports::golem::component_counter_stub::stub_counter::Guest
for Component {
    type ApiCounter = crate::ApiCounter;
    type FutureGetResult = crate::FutureGetResult;
}
impl crate::bindings::exports::golem::component_counter_stub::stub_counter::GuestFutureGetResult
for FutureGetResult {
    fn subscribe(&self) -> bindings::wasi::io::poll::Pollable {
        let pollable = self.future_invoke_result.subscribe();
        let pollable = unsafe {
            bindings::wasi::io::poll::Pollable::from_handle(pollable.take_handle())
        };
        pollable
    }
    fn get(&self) -> Option<u64> {
        self.future_invoke_result
            .get()
            .map(|result| {
                let result = result
                    .expect(
                        &format!(
                            "Failed to invoke remote {}",
                            "golem:component-counter/api-counter.{get}"
                        ),
                    );
                (result
                    .tuple_element(0)
                    .expect("tuple not found")
                    .u64()
                    .expect("u64 not found"))
            })
    }
}
impl crate::bindings::exports::golem::component_counter_stub::stub_counter::GuestApiCounter
for ApiCounter {
    fn new(location: crate::bindings::golem::rpc::types::Uri) -> Self {
        let location = golem_wasm_rpc::Uri {
            value: location.value,
        };
        Self {
            rpc: WasmRpc::new(&location),
        }
    }
    fn blocking_add(&self, value: u64) -> () {
        let result = self
            .rpc
            .invoke_and_await(
                "golem:component-counter/api-counter.{add}",
                &[WitValue::builder().u64(value)],
            )
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}",
                    "golem:component-counter/api-counter.{add}"
                ),
            );
        ()
    }
    fn add(&self, value: u64) -> () {
        let result = self
            .rpc
            .invoke(
                "golem:component-counter/api-counter.{add}",
                &[WitValue::builder().u64(value)],
            )
            .expect(
                &format!(
                    "Failed to invoke remote {}",
                    "golem:component-counter/api-counter.{add}"
                ),
            );
        ()
    }
    fn blocking_get(&self) -> u64 {
        let result = self
            .rpc
            .invoke_and_await("golem:component-counter/api-counter.{get}", &[])
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}",
                    "golem:component-counter/api-counter.{get}"
                ),
            );
        (result.tuple_element(0).expect("tuple not found").u64().expect("u64 not found"))
    }
    fn get(
        &self,
    ) -> crate::bindings::exports::golem::component_counter_stub::stub_counter::FutureGetResult {
        let result = self
            .rpc
            .async_invoke_and_await("golem:component-counter/api-counter.{get}", &[]);
        crate::bindings::exports::golem::component_counter_stub::stub_counter::FutureGetResult::new(FutureGetResult {
            future_invoke_result: result,
        })
    }
    fn blocking_run_all_tasks(&self) -> () {
        let result = self
            .rpc
            .invoke_and_await("golem:component-counter/api-counter.{run-all-tasks}", &[])
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}",
                    "golem:component-counter/api-counter.{run-all-tasks}"
                ),
            );
        ()
    }
    fn run_all_tasks(&self) -> () {
        let result = self
            .rpc
            .invoke("golem:component-counter/api-counter.{run-all-tasks}", &[])
            .expect(
                &format!(
                    "Failed to invoke remote {}",
                    "golem:component-counter/api-counter.{run-all-tasks}"
                ),
            );
        ()
    }
}
bindings::export!(Component with_types_in bindings);
