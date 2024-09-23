#![allow(warnings)]
use golem_wasm_rpc::*;
#[allow(dead_code)]
mod bindings;
pub struct ApiGolemAsync {
    rpc: WasmRpc,
}
impl ApiGolemAsync {}
pub struct FutureGetResult {
    pub future_invoke_result: FutureInvokeResult,
}
struct Component;
impl crate::bindings::exports::golem::component_golem_async_stub::stub_golem_async::Guest
for Component {
    type ApiGolemAsync = crate::ApiGolemAsync;
    type FutureGetResult = crate::FutureGetResult;
}
impl crate::bindings::exports::golem::component_golem_async_stub::stub_golem_async::GuestFutureGetResult
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
                            "golem:component-golem-async/api-golem-async.{get}"
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
impl crate::bindings::exports::golem::component_golem_async_stub::stub_golem_async::GuestApiGolemAsync
for ApiGolemAsync {
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
                "golem:component-golem-async/api-golem-async.{add}",
                &[WitValue::builder().u64(value)],
            )
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}",
                    "golem:component-golem-async/api-golem-async.{add}"
                ),
            );
        ()
    }
    fn add(&self, value: u64) -> () {
        let result = self
            .rpc
            .invoke(
                "golem:component-golem-async/api-golem-async.{add}",
                &[WitValue::builder().u64(value)],
            )
            .expect(
                &format!(
                    "Failed to invoke remote {}",
                    "golem:component-golem-async/api-golem-async.{add}"
                ),
            );
        ()
    }
    fn blocking_get(&self) -> u64 {
        let result = self
            .rpc
            .invoke_and_await("golem:component-golem-async/api-golem-async.{get}", &[])
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}",
                    "golem:component-golem-async/api-golem-async.{get}"
                ),
            );
        (result.tuple_element(0).expect("tuple not found").u64().expect("u64 not found"))
    }
    fn get(
        &self,
    ) -> crate::bindings::exports::golem::component_golem_async_stub::stub_golem_async::FutureGetResult {
        let result = self
            .rpc
            .async_invoke_and_await(
                "golem:component-golem-async/api-golem-async.{get}",
                &[],
            );
        crate::bindings::exports::golem::component_golem_async_stub::stub_golem_async::FutureGetResult::new(FutureGetResult {
            future_invoke_result: result,
        })
    }
    fn blocking_run_all_tasks(&self) -> () {
        let result = self
            .rpc
            .invoke_and_await(
                "golem:component-golem-async/api-golem-async.{run-all-tasks}",
                &[],
            )
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}",
                    "golem:component-golem-async/api-golem-async.{run-all-tasks}"
                ),
            );
        ()
    }
    fn run_all_tasks(&self) -> () {
        let result = self
            .rpc
            .invoke("golem:component-golem-async/api-golem-async.{run-all-tasks}", &[])
            .expect(
                &format!(
                    "Failed to invoke remote {}",
                    "golem:component-golem-async/api-golem-async.{run-all-tasks}"
                ),
            );
        ()
    }
}
bindings::export!(Component with_types_in bindings);
